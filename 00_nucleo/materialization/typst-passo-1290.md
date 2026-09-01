# Passo 1290 — `repr` público e morfologia de `math.csc`/`math.dim`

> Documento de execução; não é Prompt L0. Frente B, iniciável em paralelo com
> P1288. Este passo é o único dono autorizado de mudanças em
> `01_core/src/compiler/eval/repr.rs` durante o lote P1288–P1291.

## Objetivo

Corrigir dois resíduos relacionados no mesmo owner:

1. quebra de linhas e vírgula final da representação pública de arrays/tuplas
   longas;
2. morfologia de `Content::MathOp`, observada em `math.csc` e `math.dim`.

A correção deve ser geral e baseada no contrato do vanilla. É proibido
especializar nomes de símbolos ou operadores para fazer somente os probes
atuais passarem.

## Estado congelado de partida

Medição anterior à decisão, árvore não commitada em
`2026-08-31T10:25:52-03:00`, `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`:

- vanilla `/usr/local/bin/typst`: SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- L0 `00_nucleo/prompts/compiler/eval/repr.md`: SHA-256
  `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c`;
- consumer `01_core/src/compiler/eval/repr.rs`: SHA-256
  `7c56b8c79f489abe568ec3a104cae79f08dab81d9be3bccae6689a15a6589bf4`;
- no consumer, `Value::Array` concatena manualmente com `", "`, apesar de
  `pretty_array_like`/`pretty_comma_list` já existirem no mesmo owner;
- `Content::MathOp` representa atualmente `op(<text>)`, descartando os nomes
  públicos `text:` e `limits:`;
- `repr((type(math.csc), repr(math.csc)))`:
  vanilla `(content, "op(text: [csc], limits: false)")`, cristalino
  `(content, "op([csc])")`;
- `math.dim` apresenta a mesma divergência;
- o inventário P1284 congelado tem SHA-256
  `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb`.

P1287 mediu 111 probes padrão, 72 iguais e 39 diferentes/desabilitados no
estado registrado pelo relatório SHA-256
`7ca2da4a24bcc04e588e28090010062d68c7b32290dda8a02da966da647e9d76`.
Essas contagens não substituem uma nova baseline no início de P1290.

## Classificação e regime

`repr` é morfologia pública da linguagem, não formatação mecânica interna.
Usar protocolo Tekt completo: contrato, oráculos, adversário, selo,
implementação/testes A/B e veredito segregados. O contrato textual precisa ser
selado antes do patch porque o implementador não pode escolher o próprio
oráculo.

É correção de paridade em fluxo contínuo pelo ADR-0127. Se a medição revelar
necessidade de alterar contrato Rust público, parar no gate humano.

## Escopo observável

### Eixo R — arrays/tuplas

- vazio, singleton e sequência curta;
- limite imediatamente abaixo, no limite e acima do threshold medido;
- itens curtos e um item interno já multilinha;
- representação externa, indentação, quebra e vírgula final;
- casos residuais de symbols/emoji/math identificados pela baseline fresca.

O threshold não pode ser inferido do número histórico “15”. Deve ser medido em
entradas sintéticas minimais contra o vanilla.

### Eixo O — operadores matemáticos

- `math.csc` e `math.dim` como testemunhas obrigatórias;
- pelo menos um operador `limits: false`, um `limits: true`, um nome composto e
  um `math.op` construído pelo utilizador;
- ordem, nomes e presença dos campos `text:` e `limits:`;
- conteúdo textual simples, markup e escaping;
- nesting dentro de array/tupla para separar a morfologia do operador da
  quebra do container.

Fora de escopo: registrar membros ausentes de `math`, mudar layout matemático,
alterar símbolos e editar o harness global durante P1288.

## Cadeia segregada obrigatória

### 1. Manifesto

Criar `00_nucleo/diagnosticos/p1290-manifest.json` com hashes do passo, L0,
baseline, binários, casos, política de `Unknown` e allowlists por papel.
Registrar estado exato da árvore e hora.

### 2. Autor do contrato

Lê somente passo, L0 vigente, vanilla pinado e medições bilaterais. Produz
contrato canônico de strings e fronteiras de quebra; não lê candidato nem
escreve código/testes.

Depois da medição, atualizar primeiro `compiler/eval/repr.md`, ressellar o hash
e invalidar qualquer contrato anterior baseado no L0 velho.

### 3. Oráculos e adversário

O autor de oráculos congela casos positivos, negativos e opacos antes de ter
acesso ao patch. O adversário precisa matar, no mínimo:

- especialização apenas de `csc`/`dim`;
- `limits: false` fixo para todos os operadores;
- omissão de `text:` ou inversão da ordem dos campos;
- threshold de quebra deslocado por um caractere;
- ausência de vírgula final no modo multilinha;
- indentação plana em nesting;
- alteração indevida do singleton ou da sequência curta.

`mutation_score` exigido: `1.0`. Mutação válida sobrevivente impede o selo.

### 4. Testes A e implementação B

Testes A partem do L0/contrato selados, sem leitura do patch, e devem falhar no
baseline cristalino. O implementador recebe somente L0, contrato e testes
selados; escreve no consumer `repr.rs` e em testes próprios, sem editar
oráculos, baseline ou veredito.

Não criar segundo formatter para arrays. Reutilização de helper existente é
decisão de implementação, aceita apenas se satisfizer o contrato medido.

### 5. Verificação

Verificar hashes antes/depois, repetição e ordem inversa. Executar testes
focais, `cargo build`, `cargo test --workspace` e `crystalline-lint .`.
Depois da estabilização do harness P1288, reexecutar toda a amostra padrão e
comparar o conjunto de antigos `MATCH`, não somente as 17 linhas esperadas.

## Coordenação com P1288 e P1291

- worktree e target dir próprios;
- P1290 possui exclusivamente `repr.rs` e seu L0;
- P1291 não pode editar nem ressellar `repr.md` enquanto P1290 estiver ativo;
- P1290 não registra funções no módulo `math`;
- alterações no harness de inventário ficam com P1288; P1290 usa fixtures e
  recibos privados até a integração;
- o veredito final de P1291 pode depender da integração de P1290, mas sua
  implementação não pode copiar o patch de P1290.

## Critério de fecho

O contrato completo de arrays/tuplas e `MathOp` precisa estar preservado nos
casos selados, com score adversarial `1.0`, RED→GREEN e lint limpo. O ganho
projetado é `+17 MATCH` na amostra congelada anterior — dois de `MathOp` e
quinze de quebra externa — sujeito à recontagem inicial. Não alegar paridade
geral a partir desse número.

