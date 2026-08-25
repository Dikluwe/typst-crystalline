# P1178 — auditoria da família HTML de documento

**Data:** 2026-08-25T16:06:48-03:00  
**Baseline:** vanilla ratificado `a51e02804`  
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`  
**Estado:** parado no gate ADR-0127

## Proveniência

Vanilla `/usr/local/bin/typst`: SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Cristalino `./target/debug/typst`: SHA-256
`9d73429527721d4bf6cda1a8ad0049c3c77f5dbfbf3cd2670cb14fe8e2ede325`.
A working tree não estava commitada: seis paths tracked acumulavam 1007
inserções e 26 remoções de P1173.1–P1177.1, além dos passos/diagnósticos
untracked correspondentes. O índice estava vazio (exit 0).

## Assinatura

| tag | parâmetros | específicos | body | classe | display |
|---|---:|---:|---|---|---|
| `html` | 77 | 0 | Content opcional | normal | block |
| `head` | 77 | 0 | Content opcional | normal | none |
| `body` | 77 | 0 | Content opcional | normal | block |
| `title` | 77 | 0 | Content opcional | escapable-raw L3 | none |

As quatro listas são os 76 globais mais body, conforme o asset pinado
`94dcb99`. Sondas confirmaram `function`, `body: none`, body `[X]` e attrs
globais. Sentinelas específicas e formas inválidas de body falharam. Todas
constroem em target paged e fora da hierarquia normativa. No cristalino, as
quatro funções estavam ausentes e os 58 bindings anteriores presentes.

## Composição documental medida

- `html` como único nó substitui todo o envelope automático;
- `body` como único nó é adotado dentro de `html` + head automáticos;
- `html`/`body` acompanhados por outro nó falham com
  `` `<TAG>` element must be the only element in the document ``;
- `head` não é adotado ou fundido: fica dentro do body automático e pode ser
  repetido;
- attrs de raízes adotadas são preservados e metadata só existe quando o
  envelope é gerado.

O cristalino genérico aninhava `html`/`body` dentro do body automático e não
aplicava exclusividade, portanto a divergência é L3, não de entidade.

## `title`

Whitespace foi preservado em modo pre: dois espaços e newlines sobreviveram,
e linebreak virou newline. Texto escapou `&`/`<`, mas manteve `>`/aspas. Filho
`span` produziu `HTML raw text element cannot have non-text children`; vazio
produziu `<title></title>`. O exporter cristalino atualmente trata `title`
como elemento normal, colapsa whitespace, aceita filhos e escapa caracteres a
mais.

## Decisão e gate

Propor exatamente `html.html`, `html.head`, `html.body` e `html.title` pela via
global-only. Nuclear separadamente em L3 adoção/exclusividade de raiz e modo
escapable-raw de `title`. A entidade atual basta; não há mudança de default ou
fase.

Os L0s foram atualizados sem resselo. Como quatro bindings públicos seriam
adicionados, P1178 para no gate ADR-0127. Após aprovação, P1178.1 fará resselo,
RED, implementação e validação GREEN somente deste escopo.
