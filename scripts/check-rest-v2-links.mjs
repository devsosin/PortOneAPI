#!/usr/bin/env node
/**
 * docs/rest-v2 의 상대 링크(.md 파일, #앵커)가 모두 실제 파일·제목·id 로 풀리는지 검사한다.
 * 사용법: node scripts/check-rest-v2-links.mjs [문서 디렉터리]   (기본: docs/rest-v2)
 * 코드 블록 안의 링크는 무시한다. 깨진 링크가 있으면 목록을 출력하고 종료 코드 1.
 */
import { readdir, readFile } from 'node:fs/promises';
import { join, dirname, resolve, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = resolve(process.argv[2] || join(dirname(fileURLToPath(import.meta.url)), '..', 'docs', 'rest-v2'));
const walk = async (d) => (await Promise.all((await readdir(d, { withFileTypes: true })).map((e) => (e.isDirectory() ? walk(join(d, e.name)) : [join(d, e.name)])))).flat();
const files = (await walk(ROOT)).filter((f) => f.endsWith('.md'));
const stripCode = (t) => t.replace(/```[\s\S]*?```/g, '');
const slug = (t) => t.toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu, '').trim().replace(/\s/g, '-');

const anchorsOf = (text) => {
  const set = new Set();
  const seen = {};
  for (const m of stripCode(text).matchAll(/^#{1,6}\s+(.*?)\s*#*\s*$/gm)) {
    const s = slug(m[1].replace(/`/g, '').replace(/\[([^\]]*)\]\([^)]*\)/g, '$1'));
    set.add(seen[s] === undefined ? s : `${s}-${seen[s]}`);
    seen[s] = (seen[s] ?? -1) + 1;
    if (seen[s] === 0) seen[s] = 1;
  }
  for (const m of text.matchAll(/<a\s+id="([^"]+)"/g)) set.add(m[1]);
  return set;
};

const texts = new Map(await Promise.all(files.map(async (f) => [f, await readFile(f, 'utf8')])));
const anchors = new Map([...texts].map(([f, t]) => [f, anchorsOf(t)]));
let bad = 0, total = 0;
for (const [f, text] of texts) {
  for (const m of stripCode(text).matchAll(/\]\(([^)\s]+)\)/g)) {
    const u = m[1];
    if (/^[a-z][a-z0-9+.-]*:/i.test(u)) continue; // http(s), mailto 등은 유지(검사 대상 아님)
    total++;
    const [p, hash] = u.split('#');
    const target = p ? resolve(dirname(f), decodeURI(p)) : f;
    let err = '';
    if (!texts.has(target)) err = '파일 없음';
    else if (hash && !anchors.get(target).has(decodeURIComponent(hash))) err = '앵커 없음';
    if (err) { bad++; console.log(`${relative(ROOT, f)}: ${u} — ${err}`); }
  }
}
console.log(`checked ${total} relative links in ${files.length} files: ${bad} broken`);
process.exit(bad ? 1 : 0);
