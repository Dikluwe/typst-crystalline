# Prompt L0 — contrato black-box integrado de linguagem e exportação
Hash do Código: 594b21b4

**Camada:** L4 — teste de integração
**Ficheiro alvo exclusivo:** `04_wiring/tests/p1293_contract.rs`
**Vanilla ratificado:** `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`
**ADRs:** ADR-0107, ADR-0108, ADR-0128, ADR-0129

## Propriedade

Este prompt possui exclusivamente o integration test acima. O consumer agrega
regressões black-box que atravessam vários owners produtivos, sem passar a
possuí-los ou duplicar suas implementações.

Compara semântica, morfologia, diagnósticos, geometria e DOM. Não usa igualdade
Rust, endereço de função ou bytes integrais de SVG/HTML como paridade.

## Infraestrutura

- verificar o hash do vanilla antes das comparações;
- executar processos com timeout finito e `NO_COLOR=1`;
- comparar exit code, stdout e stderr quando são observáveis;
- repetir casos em ordem direta e inversa;
- usar temporários exclusivos com limpeza RAII;
- tratar timeout, crash, ferramenta ausente ou output ilegível como falha.

## A — `float.is-nan`

Verificar identidade e `repr` da função estática, chamadas estáticas e ligadas
sobre NaN, finitos e infinitos, além de aridade, named desconhecido, receiver
incompatível, acesso ligado sem chamada e casts. O transcript coincide com o
vanilla.

## B — constructors matemáticos

Verificar `math.attach`, `math.binom`, `math.mono` e `math.script`: identidade,
`repr`, named, defaults, slot omitido/`none`/markup vazio, convergência entre
sintaxe e chamada qualificada, erros e layout display/inline. Binom não desenha
barra de fração e `cramped` preserva seu efeito.

Layout compara `viewBox` dentro da tolerância e contagens semânticas; não IDs ou
bytes SVG completos.

## C — HTML

Com feature HTML, verificar `button`, `col`, `iframe`, `select`, `template`,
`video` e `wbr`: identidade, `repr`, campos, atributos globais, presença/ordem,
tipos e enums. Rejeitar named desconhecido, `data-*` genérico e campo cruzado.

DOM preserva ordem, nesting e escaping; `col`/`wbr` não têm closing tag. Feature
e target são independentes. Os modos `crystalline` e `vanilla` preservam o
mesmo DOM, com regras de escaping distintas; default é `crystalline`; a flag é
exclusiva de `compile` e neutra para PDF/PNG/SVG.

A fachada `typst_infra::export` reexporta diretamente
`HtmlSerializationMode`, `export_html` e `export_html_with_serialization`.

## D — namespaces grid/table

Verificar nomes curtos de `cell`, `header`, `footer`, `hline` e `vline` em
`grid` e `table`, chamadas diretas, `with`, aliases flat existentes e ausência
dos aliases flat de `hline`/`vline`. Preservar os transcripts de aridade e o
comportamento vigente de `hline`/`vline`.

## Fronteiras opacas

Bit pattern de NaN, bytes/IDs internos de SVG, browser/CSS/media/rede além do
DOM e endereço de function pointer ficam fora do observável. A tabela interna
de corrupções liga mudanças negativas a testemunhas, mas quantidade ou
percentual não são contrato arquitetural.

## Aceitação

Os testes A–D passam, o vanilla conserva a polaridade esperada e as ordens
direta/inversa são estáveis. Não há alegação fora desses observáveis.
