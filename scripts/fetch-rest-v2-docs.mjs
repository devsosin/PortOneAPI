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
 * 출력: 카테고리 폴더별 배치(아래 CATS). 폴더마다 README.md, 루트에 README.md.
 *   00-basics/  payment/  cancel-refund/  webhook/  billing-subscription/  cash-receipt-tax-invoice/
 *   promotion/  identity-verification/  platform/  reconciliation/
 * 새 엔드포인트 카테고리가 OpenAPI 에 생기면 UNIT_FILE·CATS 에 추가해야 한다(없으면 오류로 멈춘다).
 */
import { mkdir, writeFile, rm } from 'node:fs/promises';
import { join, dirname, basename } from 'node:path';
import { fileURLToPath } from 'node:url';

const BASE = 'https://developers.portone.io';
const SPEC_URL = `${BASE}/schema/v2.openapi.json`;
const OVERVIEW_URL = `${BASE}/api/rest-v2/overview`;
const INDEX_URL = `${BASE}/api/rest-v2?v=v2`;
// 가이드: [slug, 원본 경로, 저장 폴더]
const GUIDES = [
  ['webhook', '/opi/ko/integration/webhook/readme-v2.md', 'webhook'],
  ['cancel', '/opi/ko/integration/cancel/v2/readme.md', 'cancel-refund'],
  ['promotion-refund', '/opi/ko/extra/promotion/refund.md', 'cancel-refund'],
  ['checkout-integration', '/opi/ko/integration/start/v2/checkout.md', 'payment'],
  ['subscription-webhook', '/opi/ko/extra/subscription/webhook.md', 'webhook'],
  ['platform-webhook-tax-invoice', '/platform/ko/guides/webhook.md', 'webhook'],
  ['server-sdk', '/sdk/ko/v2-server-sdk/readme.md', '00-basics'],
  ['backward-compatibility', '/api/backward-compatibility.md', '00-basics'],
];

// 엔드포인트 단위(unit) -> 출력 파일. unit 은 OpenAPI x-portone-category(구독은 'subscription', 플랫폼은 경로로 3분할).
const UNIT_FILE = {
  auth: '00-basics/auth.md',
  common: '00-basics/common.md',
  payment: 'payment/payment.md',
  paymentSession: 'payment/paymentSession.md',
  checkoutProfile: 'payment/checkoutProfile.md',
  'payment.additionalFeature': 'payment/additionalFeature.md',
  pgSpecific: 'payment/pgSpecific.md',
  'payment.billingKey': 'billing-subscription/billingKey.md',
  'payment.paymentSchedule': 'billing-subscription/paymentSchedule.md',
  subscription: 'billing-subscription/subscription.md',
  'payment.cashReceipt': 'cash-receipt-tax-invoice/cashReceipt.md',
  'b2b.taxInvoice': 'cash-receipt-tax-invoice/taxInvoice.md',
  'b2b.counterparty': 'cash-receipt-tax-invoice/counterparty.md',
  'payment.promotion': 'promotion/promotion.md',
  identityVerification: 'identity-verification/identityVerification.md',
  'platform.policy': 'platform/policy.md',
  'platform.partner': 'platform/partner.md',
  'platform.settlement': 'platform/settlement.md',
  reconciliation: 'reconciliation/reconciliation.md',
};
// 카테고리 문서 제목 덮어쓰기(OpenAPI 제목이 여러 카테고리를 묶거나 없을 때)
const UNIT_TITLE = {
  subscription: '구독 API',
  'platform.policy': '파트너 정산: 정책·계약',
  'platform.partner': '파트너 정산: 파트너·계좌·사업자·설정',
  'platform.settlement': '파트너 정산: 정산·지급·이체 내역',
};
const BASICS = { overview: '00-basics/overview.md', errors: '00-basics/errors.md', schemasCommon: '00-basics/schemas-common.md' };

// 카테고리 폴더. key: README 에 올릴 주요 엔드포인트('METHOD /path', 없으면 전부).
const CATS = [
  { dir: '00-basics', title: '기본', desc: 'API 호출 전에 알아야 할 것: 호스트·인증 헤더·멱등 키·타임아웃(개요), 인증 토큰 API, 전체 에러 코드, 여러 문서가 공유하는 스키마, 하위호환성, 서버 SDK.',
    units: ['auth', 'common'], extra: ['overview', 'errors', 'schemasCommon'],
    key: ['POST /login/api-secret', 'POST /token/refresh'] },
  { dir: 'payment', title: '결제', desc: '결제 조회·승인·확정·가상계좌/에스크로·웹훅 재발송, 결제 세션, 체크아웃 프로필, PG사별 API, 인증 결제 연동 가이드. 취소 API 는 payment.md 에 있고 cancel-refund 에서 링크한다.',
    units: ['payment', 'paymentSession', 'checkoutProfile', 'payment.additionalFeature', 'pgSpecific'],
    key: ['GET /payments/{paymentId}', 'GET /payments', 'POST /payments/{paymentId}/instant', 'POST /payments/{paymentId}/confirm', 'POST /payments/{paymentId}/capture', 'POST /payments/{paymentId}/pre-register', 'POST /payments/{paymentId}/resend-webhook'] },
  { dir: 'cancel-refund', title: '취소·환불', desc: '결제 취소(환불)와 프로모션 결제 취소 가이드. 취소 엔드포인트 자체는 payment/payment.md 에 있어 앵커로 연결한다.',
    units: [], key: ['POST /payments/{paymentId}/cancel', 'POST /payments/{paymentId}/cancellations/{cancellationId}/stop'] },
  { dir: 'webhook', title: '웹훅', desc: '결제·구독·정산 웹훅 수신과 검증 가이드. 웹훅 재발송 API 는 payment 의 resend-webhook.',
    units: [], key: ['POST /payments/{paymentId}/resend-webhook'] },
  { dir: 'billing-subscription', title: '빌링키·결제 예약·구독', desc: '정기결제: 빌링키 발급·조회·삭제, 빌링키 결제, 결제 예약, 구독 상품·고객·할인·인보이스.',
    units: ['payment.billingKey', 'payment.paymentSchedule', 'subscription'],
    key: ['POST /billing-keys', 'GET /billing-keys/{billingKey}', 'DELETE /billing-keys/{billingKey}', 'POST /payments/{paymentId}/billing-key', 'POST /payments/{paymentId}/schedule', 'GET /payment-schedules'] },
  { dir: 'cash-receipt-tax-invoice', title: '현금영수증·세금계산서', desc: '현금영수증 발급·조회·취소, B2B 세금계산서 발행과 거래처 관리.',
    units: ['payment.cashReceipt', 'b2b.taxInvoice', 'b2b.counterparty'],
    key: ['POST /cash-receipts', 'GET /payments/{paymentId}/cash-receipt', 'POST /payments/{paymentId}/cash-receipt/cancel', 'GET /cash-receipts'] },
  { dir: 'promotion', title: '프로모션', desc: '프로모션 조회 API. 프로모션 결제 취소는 cancel-refund 가이드.',
    units: ['payment.promotion'], see: [['프로모션 결제 취소 가이드', 'cancel-refund/promotion-refund.md']] },
  { dir: 'identity-verification', title: '본인인증', desc: '본인인증 요청·확인·조회·SMS 재전송.',
    units: ['identityVerification'] },
  { dir: 'platform', title: '파트너 정산(플랫폼)', desc: '파트너 정산 API(약 67개). 정책·계약, 파트너·계좌·사업자·설정, 정산·지급·이체 내역 세 문서로 나눴다. 세금계산서 웹훅은 webhook 가이드.',
    units: ['platform.policy', 'platform.partner', 'platform.settlement'],
    key: ['POST /platform/partners', 'GET /platform/partners', 'POST /platform/contracts', 'GET /platform/contracts', 'POST /platform/transfers/order', 'GET /platform/transfer-summaries', 'GET /platform/partner-settlements', 'GET /platform/payouts'],
    see: [['세금계산서 웹훅 가이드', 'webhook/platform-webhook-tax-invoice.md']] },
  { dir: 'reconciliation', title: '대사', desc: '대사(정산 대조) 서비스 API.',
    units: ['reconciliation'] },
];
const DIR_SCHEMAS = (dir) => `${dir}/schemas.md`;

// 상대 경로 계산(from·to 는 OUT 기준 경로). 같은 파일이면 ''.
const relPath = (from, to) => {
  if (from === to) return '';
  const a = dirname(from).split('/').filter((x) => x !== '.');
  const b = to.split('/');
  let i = 0;
  while (i < a.length && i < b.length - 1 && a[i] === b[i]) i++;
  return [...a.slice(i).map(() => '..'), ...b.slice(i)].join('/');
};
// GitHub 앵커 규칙(소문자, 구두점 제거, 공백→-)
const ghSlug = (t) => t.toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu, '').trim().replace(/\s/g, '-');

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
  const platformUnit = (path) =>
    /^\/platform\/(additional-fee-policies|contracts|discount-share-polic)/.test(path) ? 'platform.policy'
      : /transfer|payout|settlement/.test(path) ? 'platform.settlement' : 'platform.partner';
  const fileOf = (cat, path) => {
    let u;
    if (!cat) u = path.startsWith('/platform') ? platformUnit(path) : undefined;
    else if (cat.startsWith('subscription')) u = 'subscription';
    else if (cat.startsWith('platform')) u = platformUnit(path);
    else u = cat;
    if (!UNIT_FILE[u]) throw new Error(`UNIT_FILE 에 없는 카테고리: ${cat ?? '(none)'} ${path}`);
    return u;
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
  // 스키마 위치: 둘 이상의 폴더가 쓰면 00-basics/schemas-common.md, 한 폴더의 둘 이상 문서가 쓰면 <폴더>/schemas.md, 아니면 그 문서 하단
  const dirOf = (u) => UNIT_FILE[u].split('/')[0];
  const usersOf = {};
  for (const [u, l] of Object.entries(used)) l.forEach((n) => (usersOf[n] ||= new Set()).add(u));
  const common = new Set();
  const dirShared = {}; // dir -> Set(schema)
  const schemaHome = {};
  for (const [n, us] of Object.entries(usersOf)) {
    const dirs = new Set([...us].map(dirOf));
    if (dirs.size > 1) { common.add(n); schemaHome[n] = BASICS.schemasCommon; }
    else if (us.size > 1) {
      const d = [...dirs][0];
      (dirShared[d] ||= new Set()).add(n);
      schemaHome[n] = DIR_SCHEMAS(d);
    } else schemaHome[n] = UNIT_FILE[[...us][0]];
  }
  const link = (n, cur) => {
    if (isErrorSchema(n)) return `[${n}](${relPath(cur, BASICS.errors)}#${anchor(n)})`;
    return `[${n}](${relPath(cur, schemaHome[n] ?? cur)}#${anchor(n)})`;
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
        const lines = body.split('\n').filter(Boolean).map((l) => '  ' + l.replace(/`([A-Za-z]+Error)`/g, (_, n) => (S[n] ? `[\`${n}\`](${relPath(cur, BASICS.errors)}#${anchor(n)})` : `\`${n}\``)));
        out.push(`- **${code}**${names.length ? '' : ' ' + esc(body)}`, ...(names.length ? lines : []));
      }
    }
    out.push('');
    return out.join('\n');
  };

  const outFiles = {};
  const fileTitle = (u) => UNIT_TITLE[u] || catTitle[u]?.title || u;
  for (const [u, list] of Object.entries(files)) {
    const cur = UNIT_FILE[u];
    const out = [`# ${fileTitle(u)}`, '', `> 출처: ${INDEX_URL} (OpenAPI: ${SPEC_URL}). Base URL \`${spec.servers[0].url}\`. 에러 코드: [errors.md](${relPath(cur, BASICS.errors)}). 공통 스키마: [schemas-common.md](${relPath(cur, BASICS.schemasCommon)}).`, ''];
    out.push('## 엔드포인트 목록', '');
    for (const o of list) out.push(`- \`${o.method} ${o.path}\` — ${esc(o.op['x-portone-title'] || o.op.summary)}${o.cat && o.cat !== u ? ` (${o.cat})` : ''}`);
    out.push('');
    for (const o of list) out.push(renderOp(o, cur));
    const local = used[u].filter((n) => schemaHome[n] === cur);
    if (local.length) {
      out.push('---', '', '# 스키마', '');
      local.sort().forEach((n) => out.push(renderSchema(n, cur)));
    }
    outFiles[cur] = out.join('\n');
  }
  for (const [d, set] of Object.entries(dirShared)) {
    const cur = DIR_SCHEMAS(d);
    outFiles[cur] = [`# 공유 스키마 (${d})`, '', '> 이 폴더의 둘 이상의 문서가 참조하는 스키마.', '', ...[...set].sort().map((n) => renderSchema(n, cur))].join('\n');
  }
  outFiles[BASICS.schemasCommon] = [
    '# 공통 스키마', '', '> 둘 이상의 카테고리 폴더가 참조하는 스키마. 한 문서(또는 한 폴더)에서만 쓰는 스키마는 그쪽에 있다.', '',
    ...[...common].sort().map((n) => renderSchema(n, BASICS.schemasCommon)),
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
    errOut.push(`- <a id="${anchor(n)}"></a>\`${n}\`: ${S[n].oneOf.map((o) => `\`${refName(o.$ref)}\``).join(', ')}`);
  outFiles[BASICS.errors] = errOut.join('\n') + '\n';

  return { outFiles, files, fileTitle, endpointCount: ops.length };
}

// ---------- main ----------
const spec = JSON.parse(await get(SPEC_URL));
const overviewMd = htmlToMd(await get(OVERVIEW_URL));
const { outFiles, files, fileTitle, endpointCount } = build(spec);

await rm(OUT, { recursive: true, force: true });
for (const c of CATS) await mkdir(join(OUT, c.dir), { recursive: true });
const w = async (name, text) => {
  await mkdir(dirname(join(OUT, name)), { recursive: true });
  await writeFile(join(OUT, name), text.endsWith('\n') ? text : text + '\n');
};

await w(BASICS.overview, `# REST API V2 개요\n\n> 출처: ${OVERVIEW_URL}\n\n${overviewMd}`);
for (const [f, text] of Object.entries(outFiles)) await w(f, text);

const guideInfo = {}; // slug -> {dir, title, url, file}
for (const [slug, path, dir] of GUIDES) {
  try {
    const md = fixGuide(await get(BASE + path), path);
    const file = `${dir}/${slug}.md`;
    await w(file, md);
    guideInfo[slug] = { dir, file, title: md.split('\n')[0].slice(2), url: BASE + path };
  } catch (e) {
    console.error('guide skipped:', path, e.message);
  }
}

// 엔드포인트 앵커 색인: 'METHOD /path' -> {file, title, anchor}
const opIndex = {};
for (const [u, list] of Object.entries(files))
  for (const o of list) opIndex[`${o.method} ${o.path}`] = { file: UNIT_FILE[u], title: o.op['x-portone-title'] || o.op.summary || '', anchor: ghSlug(`${o.method} ${o.path}`) };

const BASICS_INFO = {
  overview: ['overview.md', '호스트, 인증 헤더, 멱등 키, 타임아웃, 하위호환성', null],
  errors: ['errors.md', '전체 에러 코드(이름, HTTP 상태, 설명)', null],
  schemasCommon: ['schemas-common.md', '여러 카테고리가 공유하는 스키마', null],
};
const unitCount = (u) => files[u]?.length ?? 0;

for (const c of CATS) {
  const here = `${c.dir}/README.md`;
  const rows = [];
  for (const k of c.extra || []) rows.push(`| [${BASICS_INFO[k][0]}](${BASICS_INFO[k][0]}) | ${BASICS_INFO[k][1]} | |`);
  for (const u of c.units) rows.push(`| [${basename(UNIT_FILE[u])}](${basename(UNIT_FILE[u])}) | ${esc(fileTitle(u))} | ${unitCount(u)} |`);
  if (outFiles[DIR_SCHEMAS(c.dir)]) rows.push(`| [schemas.md](schemas.md) | 이 폴더의 문서들이 공유하는 스키마 | |`);
  const guides = GUIDES.filter((g) => g[2] === c.dir && guideInfo[g[0]]).map((g) => guideInfo[g[0]]);
  const out = [`# ${c.title}`, '', c.desc, ''];
  if (rows.length) out.push('## 파일', '', '| 파일 | 내용 | 엔드포인트 |', '|---|---|---|', ...rows, '');
  if (guides.length) out.push('## 가이드', '', '| 파일 | 제목 | 원본 |', '|---|---|---|', ...guides.map((g) => `| [${basename(g.file)}](${basename(g.file)}) | ${g.title} | ${g.url} |`), '');
  const keys = c.key || c.units.flatMap((u) => files[u].map((o) => `${o.method} ${o.path}`));
  if (keys.length) {
    out.push('## 주요 엔드포인트', '');
    for (const k of keys) {
      const e = opIndex[k];
      if (!e) throw new Error(`${c.dir}: key 엔드포인트가 OpenAPI 에 없다: ${k}`);
      out.push(`- [\`${k}\`](${relPath(here, e.file)}#${e.anchor}) — ${esc(e.title)}`);
    }
    out.push('');
  }
  if (c.see) out.push('## 관련', '', ...c.see.map(([t, p]) => `- [${t}](${relPath(here, p)})`), '');
  await w(here, out.join('\n'));
}

const catRows = CATS.map((c) => {
  const n = c.units.reduce((a, u) => a + unitCount(u), 0);
  return `| [${c.dir}/](${c.dir}/README.md) | ${c.title} | ${c.desc.split('. ')[0].replace(/\.$/, '')} | ${n || ''} |`;
});
await w('README.md', `# PortOne REST API V2 문서 (오프라인 사본)

- 원본: ${INDEX_URL}
- 수집일: ${FETCH_DATE}
- 엔드포인트 수: ${endpointCount}, OpenAPI 소스: ${SPEC_URL}
- 이 폴더는 \`scripts/fetch-rest-v2-docs.mjs\` 가 생성한다. 손으로 고치지 말고 스크립트를 다시 실행한다(카테고리 배치는 스크립트의 \`UNIT_FILE\`·\`GUIDES\`·\`CATS\`).

## 다시 만들기

\`\`\`sh
# 저장소 루트에서. Node 18 이상, 별도 의존성 없음
FETCH_DATE=YYYY-MM-DD node scripts/fetch-rest-v2-docs.mjs
node scripts/check-rest-v2-links.mjs   # 상대 링크·앵커 검사
\`\`\`

## 수집 방식

원본 페이지는 JS 렌더링(SolidStart)이라 HTML 은 엔드포인트별 본문을 그대로 담지 않는다. 같은 사이트의 OpenAPI JSON 이 렌더링 원본이므로 엔드포인트·파라미터·스키마·에러 코드는 OpenAPI 에서, 개요는 개요 페이지 HTML 에서, 가이드는 사이트가 제공하는 \`.md\` 에서 가져온다. 가이드의 링크는 절대 URL 로 바꿨다.

## 카테고리

한 주제의 API 레퍼런스와 관련 가이드를 한 폴더에 모았다. 각 폴더의 README.md 에 파일 목록과 주요 엔드포인트가 있다.

| 폴더 | 카테고리 | 내용 | 엔드포인트 |
|---|---|---|---|
${catRows.join('\n')}

에러 코드는 [00-basics/errors.md](00-basics/errors.md), 공통 스키마는 [00-basics/schemas-common.md](00-basics/schemas-common.md).
`);
console.log(`done: ${OUT}`);
