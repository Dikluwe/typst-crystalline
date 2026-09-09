# P1329 — revisão preliminar de medição e ownership

Revisor: agente `/root/p1329_review`, ambiente partilhado, revisão somente-leitura
dos artefatos produtivos; escrita restrita a `diagnosticos/p1329-review-*`.
Regime A/B, executado sem atestação de isolamento e sem refinement seal.
O contexto recebido descreveu intenção e limites; não contém patch P1329.
Skill e ambas as referências operacionais foram lidas integralmente.
Busca textual em `00_nucleo/adr/` não localizou ADR de segregação.

## Entradas e proveniência

Baseline `p1329-baseline.json` conferido com SHA-256
`d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`.
Registra HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, UTC inicial `2026-09-09T12:08:08.824769+00:00`, diff/stat,
inventário produtivo e argv/resultados por sonda. As medições abaixo são
desse baseline, não medições de candidato.

Na leitura preliminar, owner e L0 atuais eram exatamente os bytes
`original_owner` e `original_prompt` incorporados no baseline:

- owner SHA-256 `7c8096519d042d4e743eb0ba0ba5b77dfaa5fc2879ff4e452a96219c168d5052`;
- L0 SHA-256 `deced74437190452be5af7ccf47d757cf7af569054f5b3b016782d48f8e45367`.

`/usr/local/bin/typst` confere SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`, igual
ao binário vanilla do baseline. O alvo é o upstream ratificado `a51e02804`
declarado pelo repositório, sem inferir identidade de versão estampada.
Fonte local `lab/typst-original/crates/typst-library/src/foundations/calc.rs`
SHA-256 `0eb0836dc8bf17ba214d04be2c14f137350766ec44b6f5afe238cf467f78f484`.

## Evidência antes da conclusão

- Vanilla `foundations/calc.rs:84-93` conserva Length, Angle, Ratio e Fraction;
  Length usa `try_abs`, com erro `cannot take absolute value of this length`.
- Vanilla `layout/length.rs:57-60` aceita somente `abs.is_zero() || em.is_zero()`.
  Não aceita misto de mesmo sinal. As sondas `2pt + 3em`, `-2pt - 3em` e
  `2pt - 3em` do baseline confirmam erro e âncora no argumento.
- Vanilla `layout/angle.rs:76`, `ratio.rs:103` e `fr.rs:49` aplicam módulo ao
  escalar e preservam a classe dimensional. Não normalizam ângulo nem limitam ratio.
- Cristalino `calc.rs:133-161` concentra o despacho e os guards atuais.
  `layout_types.rs:1162-1287` já oferece representação e construtores suficientes;
  não é necessário acrescentar método, entidade, trait ou resolução contextual.
- `args.rs:16-20,53-63` conserva `value_span`; `call_dispatch.rs:1058-1064`
  funde as ocorrências de With sem substituir a origem pelo span final.
  `call_dispatch.rs:1031-1049` suprime trace contido e mantém chamada externa.
  Baseline With/arguments mistos demonstra a origem externa no vanilla; o nome
  externo cristalino vigente é `calc.abs`, enquanto vanilla usa `abs`.
- `p1328-ab-tests-r2.rs:327-344` registra os casos dimensionais negativos que
  precisam de sucessão no módulo test-only integrado; o snippet histórico continua
  evidência imutável.

## Conclusão preliminar e obrigações pendentes

Inferência sustentada: o owner `stdlib/calc.rs` basta para esta correção de
paridade, em fluxo contínuo ADR-0127. Origem perdida numa rota real obrigatória,
necessidade de outro owner ou mudança de fase refutaria a inferência.

Antes de liberar implementação, o L0 P1329 deve suplantar explicitamente o
scope-out dimensional P1328, atualizar a tabela de `calc_abs` e declarar o erro
local com `value_span` real/detached quando indisponível, mantendo guards,
overflow Int, conteúdo, outros tipos e trace externo do baseline.

A política IEEE precisa ser inequívoca: abs Float já propaga NaN/Inf;
ADR-0101 enumera os consumidores de `guard_float` e abs não está nessa lista.
Aplicar guard novo às dimensões não é exigido por essa lista. NaN, Inf e zero
assinado devem ser examinados em valores efetivamente entregues à função.
As sondas `(0.0/0.0)*unidade` falham antes de abs nos dois binários; a sonda
`-calc.inf * 1fr` também não chega a abs no cristalino. Não podem fechar
comportamento IEEE de abs; controles unitários diretos podem medir esse seam.

Pendente: revisão do L0 P1329 congelado, RED dos testes independentes, comparação
final com o baseline, preservação histórica e gates. Este parecer não aprova
candidato nem declara paridade geral.
