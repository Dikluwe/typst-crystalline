# R1 — reflexão de Sequence e conteúdo vazio

Etapa concluída na working tree não commitada do HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`. Freeze de validação: **2026-10-18T01:25:18.632237+00:00**. Vanilla pinado em `bd5ebec285505889e8ff1d4f68804ab147a699f3`; SHA do oráculo `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Binário debug Tekt validado: `e4930779bb3f24de492b3f9238a3bbe027d385ea7037f0e39dd2ee3ca21b6d9f`. O diffstat e a lista exata de arquivos alterados estão no freeze/final-state; o patch desta etapa está separado do trabalho A1 e da correção de pilha.

## Correções

- `children` passou a ser campo conhecido de Empty e Sequence, inclusive vazio. O array conserva os filhos imediatos, sua ordem e seus wrappers.
- `fields()` inclui somente `children` para essas duas variantes. `has()` e `at()` usam a mesma projeção do getter.
- A identidade de linguagem do conteúdo vazio passou de `empty` para `sequence`, corrigindo o nome retornado por `func()` e as mensagens de campo inexistente. A assinatura pública e o Debug Rust foram preservados.
- Text vazio continua texto, Styled comum permanece opaco, aliases conservam os conteúdos anteriores e sequências com um filho mantêm sua identidade.

Os quatro L0s proprietários foram atualizados antes do código. A primeira tentativa de teste falhou por escopo de MockWorld, sem crédito de RED; o harness foi corrigido. O RED funcional registrou quatro falhas e um controle verde antes da implementação. Os mesmos cinco testes passaram depois. As 10 sondas bilaterais de reparo e os dois controles de Text/Styled foram aprovados na auditoria final. Sondas e fontes exatas estão em before/after e before-sources. Hashes foram ressellados após ownership/núcleos válidos. Nenhum contrato Rust público novo ou fase foi introduzido.

## Validação e RAM

**8955 testes passaram, 0 falharam, 7 ignorados; 5 doctests passaram.** Build workspace, Clippy de todos os targets com warnings proibidos, Rustdoc com warnings proibidos, fmt, V1/V5/V15/V26, linhagem reversa e diff-check passaram. Doctests foram executados em invocação separada conforme a receita validada anteriormente.

Todas as execuções pesadas foram sequenciais, com um job Cargo, **MemoryMax 12 GiB, MemoryHigh 8 GiB e swap zero**. Pico da etapa: **4.349 GiB** (4670156800 bytes). Eventos acumulados: `{'low': 0, 'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}`. Esta medição é de cgroup; não demonstra ganho de RSS do produto. Receita/horários/ambiente/comandos e resultados estão em resources/evidence.

Linter global: `{'V16:error': 106, 'V19:note': 1107, 'V20:note': 874}`. O diferencial por regra/consumer/mensagem não adicionou erros ou warnings; os V16 anteriores continuam bloqueando o gate global. As notas V19/V20 não foram silenciadas.

A única nota nova é V19 em `01_core/src/compiler/eval/bindings/field_access/content_methods.rs`: o braço `Content::Empty | Content::Sequence(_)` compartilha o schema children. O L0 proprietário, seção R1, prescreve esse agrupamento porque ambos expõem o mesmo campo requerido; os testes exercitam Empty, Sequence com um filho e Sequence com vários filhos. A nota foi mantida, sem allow/expect ou supressão. A contagem V19 passou de 1106 para 1107; V16 permaneceu 106 e V20 permaneceu 874.

## Pendências explícitas

R1 não encerra a reflexão completa de Sequence. As sondas ainda registram diferenças na chamada do construtor retornado por func(), em selector/counter e na mensagem de método inválido. R2 está nomeada nos L0s para completar construção e admissão em selector/where/show/counter, após medição dos argumentos e identidades; nenhuma função será admitida apenas pelo nome. As sondas JSON foram preservadas, mas comparação do texto diagramado não foi usada como prova de igualdade do JSON.

As pendências A1 e de pilha documentadas nas etapas anteriores, os V16 restantes e os demais lotes de paridade continuam abertos. Este relatório encerra apenas R1. Sem commit, push, publicação ou release nesta etapa.

Evidências: before.json.gz, after.json.gz, before-sources.json.gz, complete-freeze.json.gz, change.patch.gz, evidence.json.gz, resources.json.gz, lint-diff.json.gz, final-state.json.gz e manifest.sha256, neste diretório.
