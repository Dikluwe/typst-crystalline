# Passo 1434 — reparar a materialização M05 e retomar o selo P1431

## Estado medido antes da decisão

Medição executada em `2026-09-17T15:36:14-03:00`, sobre HEAD limpo
`472959d392aca2128b3bebf064bc6f66be795efd`, tree
`c3528026a49e17f1a4ccccf12b6d41e64108ed8d`. `git status --short` não
produziu linhas.

O veredito independente R7
`p1431-verifier-r7-nonseal.json` (SHA-256 `74a6a707…`) reproduziu o bloqueio:

- a receita M05 exige somente desativar o preenchimento de linhas ragged com
  `Content::Empty`, preservando parsing e restante fluxo;
- o patch histórico remove o loop `row.resize`, mas também renomeia
  `columns` para `_columns`;
- o candidato P1431 passou a usar `columns` na validação de offsets verticais;
- o mutante falha com E0425 antes de P05/P06/P23 executarem e, portanto, é
  `Unknown`, sem crédito.

Identidades congeladas na medição:

| artefato | SHA-256 |
|---|---|
| desenho R7 | `e050efb35c837e7d221c7c0416d7c6841782e6c7beb268701c50b2b59a0711a7` |
| receita M05 | `036503788d5f30d9b2183afcba8907512e7dcdff20ad9832a987b0bddad649e2` |
| veredito R7 | `74a6a7073ab122c7aa9548f67a99626cb54fe9e4d016dedcb8a1412c2fa0187a` |

## Classificação e decisão

É defeito da materialização adversarial, não do candidato nem dos oráculos.
A receita já congela a intenção correta; o patch físico excedeu-a ao alterar
um binding ainda necessário.

M05 R8 deve preservar literalmente:

```rust
let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
```

e remover somente:

```rust
for row in &mut rows {
    row.resize(columns, Content::Empty);
}
```

Nenhum teste, código produtivo, contrato, manifesto ou outro mutante pode ser
alterado para acomodar M05.

## Cadeia invalidada e escopo autorizado

A mudança invalida desenho R7, pré-selo R7, patches/recibos R7 e vereditos
posteriores. Permanecem congelados manifesto, contrato, oráculos R6,
candidato produtivo, runner público, M26 R7 e as outras 28 receitas.

Fluxo autorizado:

1. publicar desenho R8, revisando apenas a identidade/fase de M05;
2. materializar o patch exato acima;
3. executar primeiro P05/P06/P23 e P04 como controle;
4. exigir compilação bem-sucedida e morte semântica atribuível;
5. publicar pré-selo R8 somente após candidato não mutado 34/34;
6. executar M01–M30 nas ordens `forward`, `forward_repeat` e `reverse`;
7. parar no primeiro sobrevivente, `Unknown`, reversão suja ou desvio de
   identidade; selar somente com score `1.0` nas três ordens.

## Budget e rendimento

A calibração consome no máximo uma revisão M05 e duas execuções focais. O
ganho exigido é passar de E0425/`Unknown` para `Violated` através de
P05/P06/P23, sem regressão do candidato ou P04. Ausência desse ganho encerra
o passo sem executar o corpus completo.

## Gate ADR-0127

Não há mudança de contrato público, comportamento por defeito, fase de
pipeline ou compatibilidade. É correção interna da materialização de um
mutante; segue fluxo contínuo sem paragem humana.

## Regime e autoridades

Aplica-se o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado.

- intenção: este passo;
- contrato e oráculos: somente leitura;
- desenho adversarial: escreve R8, receita/patch M05 e verificador mecânico;
- pré-selo: vincula identidades e calibração focal;
- executor: executa patches sem editar entradas protegidas;
- verificador final: escreve somente recibo de selo ou não-selo.

## Gates de fecho

- M05 compila e é rejeitado semanticamente por witness causal;
- candidato não mutado 34/34;
- 30/30 famílias rejeitadas em cada uma das três ordens;
- zero sobreviventes, zero `Unknown` inesperado e reversão limpa;
- `mutation_score = 1.0`;
- `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e
  `git diff --check` verdes;
- certificado limitado ao fragmento observável P1431.
