# Passo 1433 — substituir o mutante equivalente M26 e concluir o selo P1431

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

Medição executada em `2026-09-17T14:24:00-03:00`, sobre HEAD limpo
`7291b062039de5f4a6d4614727cd3b252f211b07`, tree
`6945241223c117d16add2ea8d0556413a5063ffe`. `git status --short` e
`git diff HEAD --stat` não produziram linhas.

O candidato produtivo continua em
`cf92c3d07c05a564b9d089e005b04e9daf03cb62`; a projeção produtiva até o
HEAD medido é byte-idêntica. Os 34 oráculos R6 passam no candidato não
mutado. A campanha forward R6 matou 26 das 30 famílias e não produziu
`Unknown` inesperado, mas parou sem selo após M26 sobreviver.

O veredito independente R6 (relatório consolidado da limpeza, identificador administrativo
`d21b953a…`) reproduziu o sobrevivente e demonstrou a causa em código:

- `Content::math_matrix_full` armazena `0.5em` no carrier quando o wrapper
  omite `column_gap` e `gap`;
- a mesma construção mantém `explicit.column_gap = false`;
- `effective_matrix_options` só copia o valor do carrier quando essa marca é
  verdadeira; caso contrário, preserva o default herdado de `0.5em`;
- portanto, trocar apenas o valor armazenado de `0.5em` para zero, como fazia
  M26 R5/R6, não altera o fragmento observável e é um mutante equivalente.

## Classificação e decisão

É defeito do desenho adversarial, não do candidato nem do oráculo P32. O
contrato O-COMPATIBILITY exige que o wrapper legado preserve a semântica do
construtor comum, inclusive a omissão herdável de `column-gap`. A witness P32
já compara identidade do carrier, largura, ascent, descent e posições de texto
entre as duas rotas.

M26 R7 passa a aplicar uma única mutação semântica não equivalente na fronteira
do wrapper: substituir o quarto argumento `None` de
`Content::math_matrix_full` por `Some(Length::ZERO)`. Isso transforma
`column-gap` omitido em zero explícito, tornando `explicit.column_gap = true`
e fazendo a cascata selecionar zero. O efeito deve ser rejeitado pela witness
P32 existente, sem alterar teste, contrato ou código produtivo.

Ficam proibidos:

- mutar apenas o literal default interno novamente;
- editar `effective_matrix_options` ou o candidato para acomodar o mutante;
- adicionar uma segunda alteração de explicitness fora da chamada pública;
- enfraquecer o denominador, remover M26 ou usar outro teste como crédito
  substituto.

## Cadeia invalidada e escopo autorizado

A revisão de M26 invalida o desenho de mutantes R5, o pré-selo R6, os patches
materializados R6, seus recibos e os vereditos posteriores. Permanecem
congelados e byte-idênticos: manifesto, contrato, oráculos R6, candidato,
runner público, 29 receitas M01–M25/M27–M30 e todos os consumers de teste.

O fluxo autorizado é:

1. publicar desenho de mutantes R7 e receita M26 R7, vinculados aos oráculos
   R6;
2. atualizar apenas o verificador mecânico de desenho para reconhecer R7 e a
   identidade R6, preservando a delegação ao runner público congelado;
3. materializar M26 R7 e executar primeiro sua witness P32 mais controles de
   fronteira;
4. se M26 morrer de forma atribuível, publicar pré-selo R7;
5. rematerializar todas as 30 famílias contra a mesma projeção produtiva e
   executar as ordens `forward`, `forward_repeat` e `reverse` em processos
   frescos;
6. exigir zero sobreviventes, zero `Unknown` inesperado, reversão limpa e
   `mutation_score = 1.0` antes do veredito independente final.

## Gate ADR-0127

Não há mudança de contrato público, comportamento por defeito, fase de
pipeline ou compatibilidade. É correção interna do desenho de mutation testing
após evidência causal reproduzida. Segue fluxo contínuo, sem nova paragem
humana.

## Regime e autoridades

Aplica-se o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado.

- intenção: este passo;
- contrato e oráculos: permanecem congelados;
- adversário de desenho: escreve somente R7, M26 R7 e o verificador mecânico;
- pré-selo: verifica identidades e a morte focal antes de autorizar campanha;
- executor adversarial: materializa e executa, sem editar entradas protegidas;
- verificador final: lê entradas e recibos selados e escreve somente o
  veredito/certificado.

## Gates de fecho

- verificação mecânica R7 verde, com 30 famílias e 34 IDs R6;
- M26 R7 aplica unicamente, falha P32 pelo efeito causal esperado e reverte
  limpo;
- candidato não mutado permanece 34/34;
- M01–M30 morrem nas três ordens, sem `Unknown` inesperado;
- `mutation_score = 1.0` e patches/recibos têm identidades reproduzíveis;
- `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e
  `git diff --check` verdes;
- certificado final limita a alegação ao fragmento P1431 contratado.
