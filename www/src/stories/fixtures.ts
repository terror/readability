import type { Article, ParseResult } from '@/lib/types';

export const article: Article = {
  byline: 'Mira Castellanos',
  content: `<div id="readability-page-1" class="page"><div>
<p>After nearly eleven hours of public comment, the city council voted seven to two early on Saturday morning to approve the long-debated riverside tram line, a twelve-stop route that will connect the old docks to the university campus by 2029.</p>
<p>The decision ends a planning process that began more than a decade ago, when a group of residents first proposed reusing the abandoned freight rail corridor along the river.</p>
<h2>What the plan includes</h2>
<ul><li>Twelve stations, four of them with covered bicycle parking</li><li>A new pedestrian bridge at the foot of Lantern Street</li><li>A maintenance depot on the former cannery site</li></ul>
<blockquote><p>This corridor has been sitting empty for thirty years. Tonight we finally decided what it is for.</p></blockquote>
<p>Not everyone was pleased. Several shop owners along <a href="https://harborledger.example/ferry-road">Ferry Road</a> argued that two years of construction would drive away customers.</p>
</div></div>`,
  excerpt:
    'The twelve-stop line will link the old docks to the university campus by 2029.',
  lang: 'en',
  length: 2431,
  publishedTime: '2026-03-14T09:30:00Z',
  siteName: 'The Harbor Ledger',
  textContent:
    'After nearly eleven hours of public comment, the city council voted seven to two early on Saturday morning to approve the long-debated riverside tram line.',
  title: 'Riverside tram line approved after marathon council session',
};

export const result: ParseResult = {
  article,
  duration: 3.2,
  readable: true,
  status: 'ok',
};
