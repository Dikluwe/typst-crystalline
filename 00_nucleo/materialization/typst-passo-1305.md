# P1305 — corrigir a repr de módulos nomeados e arrays longos

**Estado:** plano de execução; nenhuma implementação P1305 realizada.
**Coorte:** `repr-module-and-large-array`, selecionada e verificada no P1304.
Este passo coordena o trabalho. Não é Prompt L0 e não legitima código sozinho.

## 1. Evidência de partida

O P1304 mediu o HEAD `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`
mais o candidato P1303 certificado, então não commitado. Sua matriz foi
executada entre `2026-09-05T02:32:36.761423+00:00` e
`2026-09-05T02:41:25.584840+00:00`. A proveniência completa, incluindo o diff
de quatro arquivos, está no manifesto abaixo; não atribuir esses resultados
ao HEAD isolado nem ao commit que venha a guardar estes documentos.

Entradas autoritativas, todas em `00_nucleo/diagnosticos/`:

| Arquivo | SHA-256 |
|---|---|
| `p1304-manifest.json` | `eeeb675a6496860dc8a12a681d64b086574fb2d7264011ac162a45b43583d862` |
| `p1304-certificate.json` | `3e6aebcbedb2eb9fec44f3fed7446a68f1f63bc8b5d01c872034b3d8a97e86e4` |
| `p1304-decision-report.md` | `92e7ecf2ac895c56831a0d4619ba79d703372cff029885bbe1101511f13425bd` |
| `p1304-owner-ledger.tsv` | `01e038177dde246c3196af79bdb36eb13eb2e03364b0b091056cb2af8f4d6770` |
| `p1304-repr-refinement.json` | `6b300e95e456fd36828e68f8b15889af2a8184e514ee5be57ad2c8c1c0b4f6d2` |

O certificado pina também catálogo, matriz, prontidão dos encoders e recibos.
Verificar a cadeia antes de reutilizá-los; não editar artefatos históricos.
O alvo é upstream/main ratificado `a51e02804`, não uma tag de versão.
O vanilla `/usr/local/bin/typst` tem SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O refinamento separou dois defeitos de representação:

- Em `01_core/src/compiler/eval/repr.rs:36`, o array representa todos os
  itens. Em `lab/typst-original/crates/typst-library/src/foundations/array.rs:1188`,
  o vanilla representa os primeiros 40 e informa quantos foram omitidos.
  Os dez mapas grandes têm 256 cores integralmente iguais nos dois lados;
  só a repr diverge. Os cinco mapas pequenos também têm dados iguais e já
  possuem repr igual. Os canais integrais e digests estão no refinamento,
  não foram deduzidos da saída abreviada.
- Em `repr.rs:56`, módulos usam `module(nome)`. O vanilla em
  `foundations/module.rs:174` usa `<module nome>` para módulos nomeados.
  `color.map` mede `module(map)` contra `<module map>`; `repr(std)` mede
  `module(std)` contra `<module global>`. `eval/mod.rs:324` constrói o
  módulo global com nome interno `std`; `bindings/field_access.rs:376`
  já projeta esse nome como `global` nos diagnósticos.

Essas fontes foram reconferidas na redação do passo. São observáveis de
morfologia da linguagem (ADR-0107); copiar a estrutura Rust, a identidade de
ponteiros ou os passos do algoritmo não é requisito. O comportamento medido
não é usado para inventar intenção histórica do upstream.

**Inferência a testar:** as duas projeções cabem no formatter existente.
Uma necessidade de nova identidade de módulo, API ou alteração de construção
refuta essa hipótese. Em particular, ainda é preciso discriminar o módulo
global de um módulo ordinário importado de um arquivo chamado `std.typ`.
A regra diagnóstica existente não prova que uma troca nominal seja segura.

## 2. Resultado pretendido e fronteira

Fechar exatamente os 12 caminhos selecionados:

`repr(std)`, `color.map`, `color.map.cividis`, `color.map.coolwarm`,
`color.map.crest`, `color.map.flare`, `color.map.icefire`, `color.map.mako`,
`color.map.rainbow`, `color.map.rocket`, `color.map.turbo`, `color.map.vlag`.

A correção é geral para a categoria representada, nunca uma lista especial
de mapas. Arrays comuns e módulos nomeados ordinários são controles dessa
generalidade, não uma segunda coorte. Os valores, ordem, cardinalidade,
lookup, feature gates e conteúdo dos módulos devem continuar preservados.

Ficam fora: encoders JSON/TOML/YAML; anonimato de plugin; mudança da entidade
Module; diagnósticos de paths externos, warnings de query e spans de panic;
layout/export; extensões intencionais; demais membros ausentes. Esses achados
continuam pendentes no diagnóstico P1304. Não os converter em paridade por
omissão nem esconder regressões novas como scope-out.

Gate esperado: `ADR-0127_CONTINUOUS_PARITY_CORRECTION`. Após medição e L0-first,
correção interna de paridade segue RED→GREEN sem nova confirmação humana.
Se precisar de campo, trait, assinatura pública, entidade, default, fase ou
quebra de compatibilidade, parar com a evidência e solicitar decisão humana.
Não reduzir silenciosamente os 12 caminhos para manter o gate contínuo.

## 3. Baseline, L0 e autoria

Ao executar, registrar HEAD real, hora, `git status --short`,
`git diff HEAD --stat`, diff e hashes dos arquivos relevantes. Um commit que
apenas consolide P1303/P1304 e este plano não é drift de produto: verificar
conteúdo e registrar a relação entre o baseline histórico e o novo HEAD.
Qualquer outra mudança deve ser explicada antes de usar o resultado P1304.

Owners permitidos para materialização:

| Prompt L0 | Consumer | SHA-256 inicial do L0 |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/repr.md` | `01_core/src/compiler/eval/repr.rs` | `555b6df222a931991f1a521b11fe2d8d58ee185cdcf90cd3f0eaf6d3c40c9355` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `01_core/src/compiler/eval/tests.rs` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |

O primeiro é o único owner produtivo; o segundo é test-only. O consumer repr
inicial tem SHA-256 `ef68c178e15c5d87d74e71c9b1bf3113851b440fc3129bf074784289f88aa9c9`.
Ler integralmente esses L0 e os contratos consultados antes da alteração.
Preservar ownership 1:1 e os pins de Núcleos; `Hash do Código`, SHA-256 do
arquivo e `@prompt-hash` efetivo não são a mesma coisa.

A execução usa a skill `tekt-materializacao-segregada`, protocolo completo,
porque precisa distinguir uma correção real de um formatter ajustado às
testemunhas. Autorizar subagentes com entradas e escritas delimitadas:

- autor do contrato/L0: baseline, fontes e observáveis; não lê patch candidato;
- autor de oráculos/testes e adversário: contrato congelado, sem patch;
- implementador: L0 e contrato selados, sem editar oráculos protegidos;
- verificador: julga resultados e escreve recibo/certificado, nunca corrige
  os artefatos que julga. Não é o autor da implementação ou do contrato.

Registrar os handoffs e capacidades num manifesto, não em uma coleção de
recibos cerimoniais. Contextos separados com filesystem compartilhado não
atestam isolamento técnico: nesse ambiente declarar explicitamente
`executado sem atestação de isolamento técnico`.

## 4. Medir e congelar o contrato antes do patch

Reutilizar os dados P1304 cuja identidade foi confirmada. Executar probes
novos apenas para fronteiras ainda não fechadas, nos perfis `default`, `html`,
`a11y` e `html+a11y` (flags reais `html` e `a11y-extras`). Congelar argv,
fixtures, cwd, binários, stdout/stderr completos, exit, hora e duração.

O contrato deve cobrir:

1. **Elisão:** arrays de 0, 1, 39, 40, 41, 42, 81 e 256 itens; valor e
   quantidade omitida exatos, pontuação, vírgulas e indentação. Incluir
   arrays comuns de números/strings, não somente cores; nesting, itens
   multilinha e um item distinto imediatamente após o limite.
2. **Não truncar dados:** comprimento, acesso aos índices 39, 40, penúltimo
   e último quando existentes, e sequência completa dos valores. Para os
   15 mapas, repetir canais integrais e digests P1304; `repr` abreviada
   nunca é prova de igualdade dos dados.
3. **Contratos anteriores:** vazio, singleton, curto, fronteira ASCII
   49/50/51 do corpo e reindentação P1290; strings escapadas, conteúdo e
   estruturas que usam o mesmo helper. Não impor elisão indiscriminada a
   todas as listas internas que apenas compartilham o formatter.
4. **Módulos:** global, `color.map`, `calc`, `sym`, `pdf` e imports nomeados.
   Medir aliases do global e imports de `std.typ`, `global.typ` e `map.typ`
   com conteúdo ordinário. Preservar o nome público de cada objeto, não o
   spelling da variável que o referencia. Proibir reconhecimento por path
   do fixture, span, nome de probe ou blacklist de testemunhas.
5. **Fronteira de serialização:** a fachada existente deve continuar
   serializando arrays estruturados integralmente; somente a repr pública
   é abreviada. Medir o caminho de fallback para módulos quando acessível,
   sem adicionar os encoders ausentes ou mudar contratos de serializers.
6. **Sentinelas fechadas:** preservar P1300/P1301r2/P1303, inclusive aliases
   negativos, lookups existentes e os três gates `pdf.*` nos quatro perfis.

Não inventar resultado esperado para os novos controles de nomes colidentes.
Se o carrier não permitir distinguir global de módulo ordinário, registrar a
contraprova e parar antes do patch: a seleção P1304 era condicionada, não
licença para heurística. Não aproveitar esse problema para resolver anonimato.

Depois da medição, atualizar os dois L0 com decisão, limites e inferência
refutável. A cláusula P1290 sobre todos os itens precisa ser explicitamente
refinada para a elisão; suas demais regras permanecem. Executar lint de
ownership/Núcleos antes do resselo. Ressellar L0 antes de escrever Rust e
confirmar `crystalline-lint --fix-hashes --dry-run .` sem reparos pendentes.

## 5. Provar que os testes distinguem a correção

Congelar testes positivos, negativos e opacos antes de entregar o patch ao
testador. RED deve falhar pelo defeito contratado; erro de build, fixture,
import ou ferramenta não vale RED. Oráculo não pode derivar a expectativa
da mesma função que verifica nem aceitar o output corrente por default.

Mutantes mínimos, cada um com testemunha e restauração em cópia temporária:

- limite 39 ou 41; ausência de elisão; contagem omitida errada;
- truncamento do array real, reordenação ou troca de um item omitido;
- elisão exclusiva dos mapas conhecidos; perda de singleton ou indentação;
- elisão aplicada indevidamente a listas de campos/conteúdo;
- wrapper antigo de Module; global exposto como std; todo módulo como global;
- correção nominal que converta também o import ordinário `std.typ`;
- alteração de lookup ou feature gate para fazer uma testemunha desaparecer.

Confirmar aplicabilidade e falha semântica de cada mutante; não contar erro
de compilação como morto. Todos os mutantes válidos devem ser rejeitados
com causa observável (`mutation_score = 1.0`). Positivos devem passar;
opacos explicitamente declarados devem permanecer `Unknown`. Nenhum caso
obrigatório ou execução desconhecida pode virar sucesso. Não incluir um
controle opaco artificial apenas para completar uma contagem.

Selar manifesto, L0, contrato e suites antes da implementação. Se algum
input protegido mudar, invalidar o selo e retomar da primeira fase afetada.
O integrador pode aplicar testes independentes congelados sem alterar seu
conteúdo; o implementador não passa a ser autor desses testes.

## 6. Implementar e revalidar

Implementar somente em `repr.rs`, com regressões permanentes no owner de
testes e/ou no `#[cfg(test)]` do próprio módulo conforme o contrato selado.
Não mudar Module, construção do std, tabelas de cor ou diagnóstico para
contornar um RED. Preservar as assinaturas e a pureza L1.

Após GREEN focal, compilar candidato fresco em target exclusivo. `/dev/shm/`
está autorizado para temporários de teste, sujeito às permissões efetivas;
usar diretório criado por `mktemp -d`, nunca um target RAM de identidade
desconhecida. Se o binário P1304 desapareceu, reconstruir seu baseline exato
em cópia isolada; registrar o novo hash em vez de fingir reutilização.

Reexecutar uma vez os 627 probes P1304 nos quatro perfis com os mesmos IDs,
além dos controles novos e sentinelas suplementares. Repetir em ordem
invertida a coorte, fronteiras, sentinelas e todos os resultados inesperados.
Não exigir igualdade de outputs que o contrato manda corrigir: separar
divergências fechadas, observáveis preservados e dívida histórica restante.
Os 12 caminhos devem atingir `MATCH_VALUE`; qualquer nova divergência tem
de ser explicada e julgada, não absorvida numa contagem agregada.

O verificador independente executa e conserva saídas de:

```bash
cargo fmt --all -- --check
cargo test -p typst-core p1305 -- --test-threads=1
cargo test -p typst-core p1290 -- --test-threads=1
cargo test -p typst-core p1300 -- --test-threads=1
cargo test -p typst-core p1301 -- --test-threads=1
cargo test -p typst-core p1303 -- --test-threads=1
cargo test --workspace
cargo build
crystalline-lint .
crystalline-lint --fail-on warning --checks v3,v4,v5,v13,v14,v15,v26 .
crystalline-lint --fix-hashes --dry-run .
git diff --check
```

Conferir que filtros focais executaram testes, não zero casos. Adicionar os
filtros de repr/serialização descobertos no contrato. Como haverá mudança de
produto/L0, o recibo workspace P1303 não substitui a execução nova. Gates
arquiteturais exigem zero violations e dry-run sem mudanças.

## 7. Entrega, custo e encerramento

Escritas de produto/L0 ficam restritas aos quatro caminhos da tabela de
owners. Registrar evidências novas somente em `00_nucleo/diagnosticos/`,
com prefixo `p1305-`: manifesto, contrato, runner/probes, medições pré/final,
ledger de mutantes, selo, recibo de verificação, certificado e relatório
final. Declarar paths exatos e logs adicionais no manifesto antes de criá-los.
Não editar P1303/P1304, lab, Núcleos, outros owners ou este plano durante a
execução sem registrar a reabertura correspondente.

Usar orçamento proporcional: no máximo duas revisões focais de contrato ou
refinamento; primeiro executar o recorte afetado e seus controles. Duas
revisões consecutivas sem mudança do vetor e da causa dominante encerram a
tentativa com diagnóstico de insuficiência, não com relaxamento do contrato.
Rebuild/corpus completo adicional só após mudança real do candidato ou de
contrato que afete esse universo, com causa e custo registrados.

Certificado e relatório formam cadeia sem auto-hash ou ciclo. Manifesto e
selo são predecessores; certificado pina evidências finais; relatório pina
certificado. Não reescrever o manifesto selado para acrescentar outputs
futuros: descrevê-los previamente e piná-los nos recibos downstream.

O relatório deve começar pelo que mudou para o usuário, mostrar antes/depois,
os controles que poderiam refutar a correção e a dívida que permanece.
Explicar qualquer hipótese refutada sobre nomes de módulos ou elisão.
Autoria, hashes e comandos sustentam a conclusão; não substituem a análise.

Veredito positivo: `P1305_PASS_REPR_MODULE_AND_LARGE_ARRAY`, somente com os
12 caminhos fechados, dados preservados, sentinelas verdes, mutantes válidos
rejeitados e validação independente completa. Caso contrário registrar
`P1305_BLOCKED` com causa concreta: baseline, contrato, identidade de módulo,
gate humano, RED inválido, mutante sobrevivente, Unknown, regressão ou gate
de verificação. Não afirmar paridade geral nem fechamento de anonimato.

Este documento prepara P1305; sua redação e commit não executam essas fases.
Na futura execução, staging/commit requerem pedido explícito do usuário.
