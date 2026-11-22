#!/usr/bin/env node
/**
 * PortOne REST API V2 문서를 compact Markdown 으로 만든다.
 *
 * 사용법 (의존성 없음, Node >= 18):
 *   node scripts/fetch-rest-v2-docs.mjs [출력 디렉터리]      # 기본: docs/rest-v2
 *
 * 환경변수: FETCH_DATE=YYYY-MM-DD  (README 에 적을 기준일. 기본: 오늘)
 *
 * 소스
 *   - https://developers.portone.io/api/rest-v2?v=v2 는 SolidStart SSR/JS 페이지이고, 엔드포인트·스키마 본문은
 *     같은 사이트가 배포하는 OpenAPI(https://developers.portone.io/schema/v2.openapi.json)에서 렌더링된다.
 *     HTML 을 긁는 것보다 정확하므로 엔드포인트·스키마·에러 코드는 OpenAPI 에서 만든다.
 *   - 개요(인증·멱등 키·타임아웃)는 https://developers.portone.io/api/rest-v2/overview 의 HTML 을 간이 변환한다.
 *   - 가이드(웹훅 검증, 결제 취소 등)는 사이트가 제공하는 `.md` 원문을 받아 링크만 절대 경로로 고친다.
 *
 * 출력: README.md, overview.md, errors.md, schemas-common.md, <카테고리>.md, guides/*.md
 */
import { mkdir, writeFile, rm } from 'node:fs/promises';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const BASE = 'https://developers.portone.io';
const SPEC_URL = `${BASE}/schema/v2.openapi.json`;
const OVERVIEW_URL = `${BASE}/api/rest-v2/overview`;
const INDEX_URL = `${BASE}/api/rest-v2?v=v2`;
const GUIDES = [
  ['webhook', '/opi/ko/integration/webhook/readme-v2.md'],
  ['cancel', '/opi/ko/integration/cancel/v2/readme.md'],
  ['promotion-refund', '/opi/ko/extra/promotion/refund.md'],
  ['checkout-integration', '/opi/ko/integration/start/v2/checkout.md'],
  ['subscription-webhook', '/opi/ko/extra/subscription/webhook.md'],
  ['platform-webhook-tax-invoice', '/platform/ko/guides/webhook.md'],
  ['server-sdk', '/sdk/ko/v2-server-sdk/readme.md'],
  ['backward-compatibility', '/api/backward-compatibility.md'],
];
const FETCH_DATE = process.env.FETCH_DATE || new Date().toISOString().slice(0, 10);
const OUT = process.argv[2] || join(dirname(fileURLToPath(import.meta.url)), '..', 'docs', 'rest-v2');

const get = async (url) => {
  const r = await fetch(url, { headers: { 'user-agent': 'rest-v2-docs-fetcher' } });
  if (!r.ok) throw new Error(`${r.status} ${url}`);
  return r.text();
};
const esc = (s = '') => String(s).replace(/\|/g, '\\|').replace(/\r?\n+/g, '<br>').trim();
const refName = (r) => r.split('/').pop();
const anchor = (n) => n.toLowerCase();

// ---------- HTML -> Markdown (개요 페이지용 간이 변환) ----------
function htmlToMd(html) {
  let h = html.replace(/<script[\s\S]*?<\/script>/g, '').replace(/<style[\s\S]*?<\/style>/g, '');
  h = (h.match(/<main[\s\S]*<\/main>/) || [h])[0];
  h = h.replace(/<nav[\s\S]*?<\/nav>/g, '').replace(/<aside[\s\S]*?<\/aside>/g, '').replace(/<svg[\s\S]*?<\/svg>/g, '');
  h = h.replace(/<a\b[^>]*?href="([^"]*)"[^>]*>([\s\S]*?)<\/a>/g, (_, href, t) => {
    const url = href.startsWith('http') ? href : href.startsWith('/') ? BASE + href : `${BASE}/api/rest-v2/${href}`;
    return `[${t.replace(/<[^>]+>/g, '').trim()}](${url})`;
  });
  h = h.replace(/<pre[\s\S]*?<\/pre>/g, (m) => '\n```\n' + m.replace(/<[^>]+>/g, '') + '\n```\n');
  h = h.replace(/<code[^>]*>([\s\S]*?)<\/code>/g, '`$1`');
  h = h.replace(/<h([1-6])[^>]*>([\s\S]*?)<\/h\1>/g, (_, n, t) => `\n\n${'#'.repeat(+n)} ${t.replace(/<[^>]+>/g, '').trim()}\n\n`);
  h = h.replace(/<li[^>]*>/g, '\n- ').replace(/<\/(p|div|ul|ol|tr)>/g, '\n').replace(/<br\s*\/?>/g, '\n');
  h = h.replace(/<[^>]+>/g, '');
  h = h.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"').replace(/&#39;/g, "'").replace(/&amp;/g, '&');
  return h.split('\n').map((l) => l.trimEnd()).join('\n').replace(/\n{3,}/g, '\n\n').trim() + '\n';
}

// ---------- 가이드(.md) 후처리 ----------
function fixGuide(md, srcPath) {
  const fm = md.match(/^---\n([\s\S]*?)\n---\n/);
  let title = '';
  if (fm) {
    title = (fm[1].match(/^title:\s*(.*)$/m) || [])[1] || '';
    md = md.slice(fm[0].length);
  }
  const dir = dirname(srcPath);
  md = md.replace(/(\]\()([^)\s]+)(\))/g, (m, a, u, c) => {
    if (/^(https?:|mailto:|#)/.test(u)) return m;
    const abs = u.startsWith('/') ? u : join(dir, u);
    return `${a}${BASE}${abs}${c}`;
  });
  md = md.replace(/<\/?(div|Hint|Tabs|TabItem)[^>]*>/g, '').replace(/\n{3,}/g, '\n\n');
  return `# ${title}\n\n> 출처: ${BASE}${srcPath}\n\n${md.trim()}\n`;
}

// ---------- OpenAPI -> Markdown ----------
function build(spec) {
  const S = spec.components.schemas;
  const isErrorSchema = (n) => {
    const s = S[n];
    if (!s) return false;
    if (s['x-portone-status-code'] !== undefined) return true;
    return !!s.oneOf && s.oneOf.every((o) => o.$ref && S[refName(o.$ref)]?.['x-portone-status-code'] !== undefined);
  };

  const refsOf = (node, acc) => {
    if (!node || typeof node !== 'object') return;
    if (node.$ref) {
      const n = refName(node.$ref);
      if (!acc.has(n)) {
        acc.add(n);
        if (!isErrorSchema(n)) refsOf(S[n], acc);
      }
      return;
    }
    for (const v of Object.values(node)) refsOf(v, acc);
  };

  // 카테고리 -> 파일
  const fileOf = (cat, path) => {
    if (!cat) return path.startsWith('/platform') ? 'platform' : 'uncategorized';
    if (cat.startsWith('subscription')) return 'subscription';
    if (cat.startsWith('platform')) return 'platform';
    return cat;
  };
  const catTitle = {};
  const walk = (list) => list?.forEach((c) => { catTitle[c.id] = c; walk(c.children); });
  walk(spec['x-portone-categories']);

  const ops = [];
  for (const [path, item] of Object.entries(spec.paths)) {
    for (const [method, op] of Object.entries(item)) {
      if (typeof op !== 'object' || !op.responses) continue;
      const cat = op['x-portone-category'];
      ops.push({ path, method: method.toUpperCase(), op, cat, file: fileOf(cat, path) });
    }
  }
  const files = {};
  for (const o of ops) (files[o.file] ||= []).push(o);

  // 스키마 사용 파일 수 계산
  const used = {};
  for (const [f, list] of Object.entries(files)) {
    const acc = new Set();
    for (const o of list) {
      refsOf(o.op.parameters, acc);
      refsOf(o.op.requestBody, acc);
      for (const [code, r] of Object.entries(o.op.responses)) if (code.startsWith('2')) refsOf(r, acc);
    }
    used[f] = [...acc].filter((n) => !isErrorSchema(n));
  }
  const count = {};
  Object.values(used).forEach((l) => l.forEach((n) => (count[n] = (count[n] || 0) + 1)));
  const common = new Set(Object.keys(count).filter((n) => count[n] > 1));

  const link = (n, cur) => {
    if (isErrorSchema(n)) return `[${n}](errors.md#${anchor(n)})`;
    const inCommon = common.has(n);
    return inCommon ? `[${n}](schemas-common.md#${anchor(n)})` : cur === 'common' ? `[${n}](#${anchor(n)})` : `[${n}](#${anchor(n)})`;
  };

  const typeStr = (s, cur) => {
    if (!s) return '';
    if (s.$ref) return link(refName(s.$ref), cur);
    if (s.oneOf) return s.oneOf.map((o) => typeStr(o, cur)).join(' \\| ');
    if (s.enum) return `enum(${s.enum.map((e) => `\`${e}\``).join(', ')})`;
    if (s.type === 'array') return `${typeStr(s.items, cur)}[]`;
    if (s.type === 'object' && s.additionalProperties) return `map<string, ${typeof s.additionalProperties === 'object' ? typeStr(s.additionalProperties, cur) : 'any'}>`;
    return s.format ? `${s.type}(${s.format})` : s.type || 'object';
  };

  const fieldsTable = (schema, cur) => {
    const props = schema.properties || {};
    const req = new Set(schema.required || []);
    const rows = Object.entries(props).map(([k, v]) => {
      const d = [v.title, v.description].filter(Boolean);
      const desc = d[0] === d[1] ? d[0] : d.join(' — ');
      return `| \`${k}\` | ${typeStr(v, cur)} | ${req.has(k) ? 'Y' : ''} | ${esc(desc)} |`;
    });
    return rows.length ? ['| 필드 | 타입 | 필수 | 설명 |', '|---|---|---|---|', ...rows].join('\n') : '';
  };

  const renderSchema = (n, cur) => {
    const s = S[n];
    const out = [`### ${n}`, ''];
    const t = s['x-portone-title'] || s.title;
    if (t) out.push(`${esc(t)}${s.description && s.description !== t ? ' — ' + esc(s.description) : ''}`, '');
    if (s.oneOf) {
      const d = s.discriminator;
      if (d) {
        out.push(`oneOf, discriminator \`${d.propertyName}\`:`, '');
        const inv = {};
        for (const [k, v] of Object.entries(d.mapping || {})) inv[refName(v)] = k;
        s.oneOf.forEach((o) => out.push(`- ${o.$ref ? `\`${inv[refName(o.$ref)] ?? ''}\` → ${link(refName(o.$ref), cur)}` : typeStr(o, cur)}`));
      } else s.oneOf.forEach((o) => out.push(`- ${typeStr(o, cur)}`));
      out.push('');
    } else if (s.enum) {
      const m = s['x-portone-enum'] || {};
      s.enum.forEach((e) => out.push(`- \`${e}\`${m[e]?.title ? ` — ${esc(m[e].title)}` : ''}`));
      out.push('');
    } else {
      const tb = fieldsTable(s, cur);
      out.push(tb || `type: ${typeStr(s, cur)}`, '');
    }
    return out.join('\n');
  };

  const renderOp = (o, cur) => {
    const { path, method, op } = o;
    const out = [`## ${method} ${path}`, ''];
    out.push(`**${esc(op['x-portone-title'] || op.summary)}** (operationId: \`${op.operationId}\`)`, '');
    const ttl = op['x-portone-title'] || op.summary || '';
    let desc = op['x-portone-description'] || (op.description || '').trim();
    if (!op['x-portone-description'] && desc.startsWith(ttl)) desc = desc.slice(ttl.length).trim();
    if (desc) out.push(desc, '');
    const sec = (op.security || []).flatMap((x) => Object.keys(x));
    if (sec.length) out.push(`인증: ${sec.map((s) => (s === 'portOne' ? '`Authorization: PortOne <API secret>`' : '`Authorization: Bearer <access token>`')).join(' 또는 ')}`, '');
    if (op.parameters?.length) {
      out.push('### 파라미터', '', '| 이름 | 위치 | 타입 | 필수 | 설명 |', '|---|---|---|---|---|');
      for (const p of op.parameters) {
        const d = (p.description || '').replace(/^.*\n\n/, '');
        out.push(`| \`${p.name}\` | ${p.in} | ${typeStr(p.schema, cur)} | ${p.required ? 'Y' : ''} | ${esc(d)} |`);
      }
      out.push('');
    }
    const body = op.requestBody?.content?.['application/json']?.schema;
    if (body) {
      const name = body.$ref ? refName(body.$ref) : null;
      out.push(`### 요청 본문 (application/json${op.requestBody.required ? ', 필수' : ''})`, '');
      if (name) {
        out.push(`타입: ${link(name, cur)}`, '');
        const sc = S[name];
        if (sc.properties) out.push(fieldsTable(sc, cur), '');
      } else {
        out.push(typeStr(body, cur), '');
        if (body.properties) out.push(fieldsTable(body, cur), '');
      }
    }
    out.push('### 응답', '');
    for (const [code, r] of Object.entries(op.responses)) {
      const sch = r.content?.['application/json']?.schema;
      if (code.startsWith('2')) {
        out.push(`- **${code}** ${esc(r.description)}${sch ? ': ' + typeStr(sch, cur) : ''}`);
      } else {
        const names = [...(r.description || '').matchAll(/`([A-Za-z]+Error)`/g)].map((m) => m[1]);
        const body = r.description || '';
        const lines = body.split('\n').filter(Boolean).map((l) => '  ' + l.replace(/`([A-Za-z]+Error)`/g, (_, n) => (S[n] ? `[\`${n}\`](errors.md#${anchor(n)})` : `\`${n}\``)));
        out.push(`- **${code}**${names.length ? '' : ' ' + esc(body)}`, ...(names.length ? lines : []));
      }
    }
    out.push('');
    return out.join('\n');
  };

  const outFiles = {};
  const fileTitle = (f) => catTitle[f]?.title || ({ subscription: '구독 API', platform: '파트너 정산 API', uncategorized: '기타' }[f] || f);
  for (const [f, list] of Object.entries(files)) {
    const out = [`# ${fileTitle(f)}`, '', `> 출처: ${INDEX_URL} (OpenAPI: ${SPEC_URL}). Base URL \`${spec.servers[0].url}\`. 에러 코드: [errors.md](errors.md). 공통 스키마: [schemas-common.md](schemas-common.md).`, ''];
    out.push('## 엔드포인트 목록', '');
    for (const o of list) out.push(`- \`${o.method} ${o.path}\` — ${esc(o.op['x-portone-title'] || o.op.summary)}${o.cat && o.cat !== f ? ` (${o.cat})` : ''}`);
    out.push('');
    for (const o of list) out.push(renderOp(o, f));
    const local = used[f].filter((n) => !common.has(n));
    if (local.length) {
      out.push('---', '', '# 스키마', '');
      local.sort().forEach((n) => out.push(renderSchema(n, f)));
    }
    outFiles[f] = out.join('\n');
  }

  outFiles['schemas-common.md'.replace('.md', '')] = [
    '# 공통 스키마', '', '> 둘 이상의 카테고리 문서가 참조하는 스키마. 한 문서에서만 쓰는 스키마는 그 문서 하단에 있다.', '',
    ...[...common].sort().map((n) => renderSchema(n, 'common')),
  ].join('\n');

  // errors
  const errs = Object.keys(S).filter((n) => S[n]['x-portone-status-code'] !== undefined).sort();
  const errOut = ['# 에러 코드', '',
    `> 출처: ${SPEC_URL} (\`x-portone-status-code\`). 에러 응답 본문은 \`{"type": "<에러 이름>", "message": "..."}\` 형태이며, \`type\` 값이 아래 이름과 같다(PgProviderError 등은 \`pgCode\`·\`pgMessage\` 가 추가됨). 엔드포인트별 발생 가능 에러는 각 카테고리 문서의 응답 절에 있다.`, '',
    '| 이름 | HTTP | 설명 | 추가 필드 |', '|---|---|---|---|'];
  for (const n of errs) {
    const s = S[n];
    const extra = Object.keys(s.properties || {}).filter((k) => k !== 'type' && k !== 'message');
    errOut.push(`| <a id="${anchor(n)}"></a>\`${n}\` | ${s['x-portone-status-code']} | ${esc(s['x-portone-title'] || s.title)} | ${extra.map((e) => `\`${e}\``).join(', ')} |`);
  }
  errOut.push('', '## 엔드포인트별 에러 묶음 (union)', '');
  for (const n of Object.keys(S).filter((n) => isErrorSchema(n) && !S[n]['x-portone-status-code']).sort())
    errOut.push(`- \`${n}\`: ${S[n].oneOf.map((o) => `\`${refName(o.$ref)}\``).join(', ')}`);
  outFiles.errors = errOut.join('\n') + '\n';

  return { outFiles, files, fileTitle, endpointCount: ops.length };
}

// ---------- main ----------
const spec = JSON.parse(await get(SPEC_URL));
const overviewMd = htmlToMd(await get(OVERVIEW_URL));
const { outFiles, files, fileTitle, endpointCount } = build(spec);

await rm(OUT, { recursive: true, force: true });
await mkdir(join(OUT, 'guides'), { recursive: true });
const w = (name, text) => writeFile(join(OUT, name), text.endsWith('\n') ? text : text + '\n');

await w('overview.md', `# REST API V2 개요\n\n> 출처: ${OVERVIEW_URL}\n\n${overviewMd}`);
for (const [f, text] of Object.entries(outFiles)) await w(`${f}.md`, text);

const guideRows = [];
for (const [slug, path] of GUIDES) {
  try {
    const md = fixGuide(await get(BASE + path), path);
    await w(`guides/${slug}.md`, md);
    guideRows.push(`| [guides/${slug}.md](guides/${slug}.md) | ${md.split('\n')[0].slice(2)} | ${BASE}${path} |`);
  } catch (e) {
    console.error('guide skipped:', path, e.message);
  }
}

const catRows = Object.entries(files).map(([f, l]) => `| [${f}.md](${f}.md) | ${fileTitle(f)} | ${l.length} |`);
await w('README.md', `# PortOne REST API V2 문서 (오프라인 사본)

- 원본: ${INDEX_URL}
- 수집일: ${FETCH_DATE}
- 엔드포인트 수: ${endpointCount}, OpenAPI 버전 소스: ${SPEC_URL}
- 이 폴더는 \`scripts/fetch-rest-v2-docs.mjs\` 가 생성한다. 손으로 고치지 말고 스크립트를 다시 실행한다.

## 다시 만들기

\`\`\`sh
# 저장소 루트에서. Node 18 이상, 별도 의존성 없음
FETCH_DATE=YYYY-MM-DD node scripts/fetch-rest-v2-docs.mjs
\`\`\`

## 수집 방식

원본 페이지는 JS 렌더링(SolidStart)이라 HTML 은 엔드포인트별 본문을 그대로 담지 않는다. 같은 사이트의 OpenAPI JSON 이 렌더링 원본이므로 엔드포인트·파라미터·스키마·에러 코드는 OpenAPI 에서, 개요는 개요 페이지 HTML 에서, 가이드는 사이트가 제공하는 \`.md\` 에서 가져온다. 링크는 절대 URL 로 바꿨다.

## 파일

| 파일 | 내용 |
|---|---|
| [overview.md](overview.md) | 호스트, 인증 헤더, 멱등 키, 타임아웃, 하위호환성 |
| [errors.md](errors.md) | 전체 에러 코드(이름, HTTP 상태, 설명) |
| [schemas-common.md](schemas-common.md) | 여러 문서가 공유하는 스키마 |

| 카테고리 문서 | 제목 | 엔드포인트 |
|---|---|---|
${catRows.join('\n')}

| 가이드 | 제목 | 원본 |
|---|---|---|
${guideRows.join('\n')}
`);
console.log(`done: ${OUT}`);
