# L0 — compiler/eval/bindings/field_access/identity
Hash do Código: c5eb3b96

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/field_access/identity.rs`. No HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` e árvore limpa, `01_core/src/compiler/eval/bindings/field_access.rs:434-556` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Contrato

Identificar funções nativas e módulos efetivos e emitir avisos de símbolos obsoletos com base no binding resolvido. Preservar as assinaturas e semântica de `00_nucleo/prompts/compiler/eval/bindings/field_access.md`; despacho estático na mesma camada L1, sem I/O nem estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de acesso a campos; `crystalline-lint .` passa com ownership e hashes corretos. Não inferir ganho de desempenho.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/`
conserva before.json.gz com UTC, status, diff HEAD --stat e hashes exatos;
baseline-reuse.json prova igualdade de fontes/configuração/binário com a
matriz final anterior de 3956 casos em oito perfis. before-batch.json.gz
mede 144 casos detalhados; before-compat.json.gz mede 456 controles de
nomes, aliases e namespace. Cada sonda conserva argv, binários e saídas.
O pin é `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

As quinze rotas restantes são rejeitadas por where/kind no Tekt e aceitas
no pin. format.rs:17–27/96–146 compartilha native_format entre os cinco
formatos, logo só ponteiro não distingue schemas. pinned-schema.json.gz
conserva nove declarações upstream com blobs/hash/linhas e atributos elem.
FuncRepr é pub(crate) em entities/func.md:21 e entities/func.rs:24; a nota
anterior de que o L0 o mostrava público foi refutada em retificacao-func-repr.md.
public-contract-before.json fixa assinaturas, reexports e structs públicos
antes da mudança; a classificação é representação interna e paridade, não
novo contrato público ou inferência de intenção por render.

### Responsabilidade exclusiva

native_public_name inclui o payload Format junto de Native, inclusive ao
descascar With. is_user_defined_closure nomeia Format como false. O braço
Native de is_format_pdf_function também aceita Format e preserva exatamente
a validação atual de nome/namespace, gates e aliases; não usar essa função
como autenticação da capacidade de elemento. Não mudar políticas dos módulos
std/format, avisos de símbolos ou diagnósticos a11y.

### Aceitação e fronteiras

Consumir as invariantes compartilhadas do Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml`. As novas
alternativas Native/Format são agrupáveis porque usam o mesmo payload/ABI
para projeção e execução; cada uma deve ser exercitada nos testes de nome,
namespace, ponteiro, Eq/Hash, clone/With e despacho. V19 novo nessas uniões
deve ser inventariado e ligado aos testes; não adicionar suppressions.
A nova variante deve ser explicitamente nomeada nos matches fechados.

Fluxo contínuo ADR-0127: nenhuma definição, assinatura, campo ou trait
externo novo, compatibilidade, default ou fase muda. Antes do código, RED
funcional dos quinze casos por factories atuais e controle de observação
real contra Native não marcada. GREEN exige corpus completo, 144 casos de
campos/chamadas e 456 controles de compatibilidade preservados fora de
where/kind. Comparar public-contract-before/after e sizeof antes/depois.

L0 antes do código; ownership/V26 antes de resselo restrito aos onze pares.
Build, suíte workspace, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26, reverse
e diferencial global sem novo erro/aviso. Comandos pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Rastrear notas V19 legítimas com cobertura.

Esta etapa fecha admissão where/kind, não todas as operações de seleção,
show/query, síntese, morfologia dos carriers ou counter. As frentes FG1, RM1,
SY1, RG1 e V16 anteriores preservam seus escopos/gates; não aplicar candidatos
antigos por cópia nem publicar.


### Cobertura direta das alternativas V19 — 2026-10-17

Medição anterior à decisão: candidate-1/state.json.gz e
differential-review.json em
lab/results/construction/diagnostics/native-format-catalogue-2026-10-17 registram oito
notas V19 novas e quatro atualizadas. Estas últimas apenas incluem Format
no padrão e elevam a contagem; não são violações removidas. O primeiro
comparador exigia mensagem idêntica e por isso recusou a atualização.

Este owner recebe testes diretos complementares; não mudar o comportamento
produtivo, suprimir notas ou usar somente um caminho como prova do grupo.
Exercitar diretamente native_public_name e is_user_defined_closure com
Native, Format, Engine, Closure, Element, Plugin e With, incluindo With de
closure e de formato. Exercitar is_format_pdf_function nos dois braços
nativos compatíveis, formatos não-PDF e nos tipos restantes; nome/namespace
continua política de campo/gate, sem autenticar capacidade de elemento.


## Gate HTML e identidade de módulos — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/html-feature-identity-2026-10-17/before.json.gz registra
UTC, status, diff HEAD --stat, fontes e L0 completos e hashes; todos os 2123
hashes da etapa anterior foram conferidos antes de reutilizar sua matriz.
before-probe.json.gz conserva argv, fixtures, outputs e SHA-256 dos binários.
O vanilla é o pin 9f2b6e8715237cb086899a42873660fe744622e8 e o binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
fields.rs:39–52 só aplica o gate html a format, não a std; identity.rs:84–101
reconhece módulos por nomes/bindings. As sondas com import correto refutam
paridade: std.html direto/alias/closure/reexport aceita Html desligada;
global.typ com type/format/document e asset comum recebe gate indevido;
format.typ com funções svg/png/pdf/html recebe gates html/bundle indevidos.
A tentativa inicial de sonda com let m = import foi inválida (import retorna
none) e está arquivada em candidate-probe-syntax; não conta como evidência.
A fonte upstream pinada module.rs:138–154 lê o guard do binding e code.rs:
347–368 ancora no field. Isto comprova semântica/diagnóstico, não exige
replicar BindingGuard ou endereços upstream. A instrução vigente no L0 pai
field_access.md:61–62 proíbe identificar std comparando bindings.

### Decisão e responsabilidade exclusiva

is_effective_std_module e is_effective_format_module delegam aos predicados internos do Module; remover autenticação por nome/type/document/svg/png/pdf/html. A política de símbolo obsoleto, PDF/a11y e identidades de funções permanece intocada. Testar com scopes nativos copiados por Module::new e por clones reais, não só nomes vazios.

Consumir o núcleo canônico pinado builtin-module-identity.toml. O marcador é
mecânica privada para cumprir gates vigentes, não novo contrato público;
ADR-0127 permite fluxo contínuo de correção/paridade. A API pública de Module
continua opaca, sem novos campos/métodos exportados; comparar declarações e
assinaturas antes/depois. Nenhuma alteração pública/fase/default é autorizada.

### Aceitação e limites

RED funcional antes da implementação; GREEN dos gates, homônimos, aliases,
closures e imports nos oito perfis; mensagem/hints/span e precedência causal.
Não atribuir crédito MATCH/Preserved de feature ativa a casos desligados:
classificar separadamente rejeição de feature e aceitação com feature ativa.
Conservar divergência prévia de igualdade entre std de arquivos distintos;
esta etapa não altera Eq de Module nem declara paridade global.
Testar a matriz anterior de caminhos std.html desligados e controles ativados;
sem publicar, commit ou release. Build, suíte workspace, Clippy all-targets e
Rustdoc estritos, fmt, V1/V5/V15/V26, reverse e diferencial global sem novo
erro/aviso. Somente resselo dos seis owners após validar ownership/núcleos.
Comandos pesados sequenciais com MemoryMax12GiB/High10GiB/swap0/jobs1.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Conservar a responsabilidade anterior; fields extrai os três gates para check_module_field_feature(&Value, &str, Span, Features) -> SourceResult<()>, interno a eval, e o chama antes do lookup. Os demais owners não duplicam os diagnósticos nem regras.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.
