# P1178.1 — materialização da família HTML de documento

**Data final:** 2026-08-25T16:17:58-03:00  
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`  
**Estado:** working tree não commitada; índice vazio

## Entrega

- `html.html`, `html.head`, `html.body` e `html.title` foram acrescentados ao
  dispatcher global-only, totalizando 62 tags;
- as quatro reutilizam 76 globais e body Content opcional;
- `html` único substitui o envelope; `body` único é adotado pelo envelope;
- `html`/`body` com siblings retornam o diagnóstico de exclusividade medido;
- `head` permanece nó comum no body automático;
- `title` preserva texto/whitespace, converte linebreak em newline, absorve o
  espaço sintático seguinte, escapa `&`/`<` e rejeita filho não textual;
- nenhuma entidade, validação no constructor, default ou fase mudou.

## RED → GREEN e decalque

O RED L1 falhou no binding `html.html`. Três REDs L3 falharam em adoção,
exclusividade e `title`. Todos passaram após a implementação, assim como a
integração CLI.

As nove fixtures válidas — documento completo, body, head, title textual,
title vazio, title com linebreak, attrs de html, attrs de body e dois heads —
foram byte-idênticas ao vanilla. Exemplos de SHA-256:

- documento completo: `bec6588ca0b1ca261e166ec840acd3105a1bb5c8401bcf985b07a8464283e978`;
- title textual: `7e1936a82a091301e0f887b9053e1a57d33daa012ea58888ede22d7e61d06c3f`.

Os três casos inválidos coincidiram em mensagem: body repetido, html com
siblings e filho não textual de title. O binário cristalino final medido tinha
SHA-256 `640975b1e9787003a8c671bbfa269bd42ed033e566496574f405d84f92a17b64`.

## Validação final

- `typst-core`: 5250 passed; 3 doctests ignored;
- `typst-infra`: 858 passed;
- `typst-shell`: 55 passed;
- `typst-wiring`: 70 testes CLI + 4 auxiliares passed;
- build, formato, `crystalline-lint` e `git diff --check`: exit 0;
- índice vazio; nenhum staging ou commit.

O resselo final tentou migrar hashes em 732 paths externos ao escopo. A
proveniência anterior provava esses paths limpos; a alteração mecânica foi
revertida somente neles, preservando os seis tracked já modificados. O linter
termina com exit 0 e os consumers deste passo estão ressellados, mas a versão
instalada passa a reportar 421 avisos V5 globais nesses paths antigos, além das
informações V19/V20 preexistentes. Nenhum deles foi incorporado silenciosamente
ao escopo P1178.1.
