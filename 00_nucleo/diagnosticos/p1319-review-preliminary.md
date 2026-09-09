# P1319 — parecer prévio sobre CSV Path/Str binário

## Proveniência e autoridade

Revisor `/root/p1319_review`, sessão compartilhada, regime A/B executado sem
atestação de isolamento técnico. Não é selo nem certificado. Produto, L0 e
oráculos somente leitura; escrita concedida somente a `p1319-review-*` em
diagnósticos. Nenhum candidato P1319 existia nas entradas recebidas.

Inspeção em 2026-09-08T15:57:36Z sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`; `git diff HEAD --stat` vazio.
Baseline executável `/tmp/p1318-target.eAgQwp/release/typst`, SHA-256
`0bdb7c2ca80d7be17775d03d3fc7ac83ba4401bf4468dd84ad7711a93b5a585c`,
recalculado nesta inspeção. Não foram executadas sondas funcionais neste parecer.

L0 `00_nucleo/prompts/compiler/stdlib/loading.md` lido integralmente, SHA-256
dos bytes `8cfa574ea121eb053598a5a45b7d54d4c1546ed920f519a3e2341b833f3e8246`.
Consumer SHA-256
`870f861651c35f44afd5f3f68548bb0364463b2ca3d4d2087b5460bdbc986f5a`.
O hash de bytes não é confundido com o hash efetivo de linhagem (`cf196a42`
no header, dependente de núcleo). Não se reexecutou lint neste parecer.

Entradas normativas lidas: skill `tekt-materializacao-segregada/SKILL.md`
e suas duas referências; instruções AGENTS fornecidas; `CLAUDE.md` e
`01_core/CLAUDE.md`; ADRs 0107, 0108, 0127 e 0129; L0s proprietários
`contracts/world.md`, `entities/path.md`, `compiler/stdlib/foundations/path.md`.
Foi consultado o trecho P1141 de `infra/system-world.md` para auditar a
semântica de leitura/include. Busca específica nas ADRs não encontrou ADR
local de materialização segregada. Nenhuma leitura/listagem de materialization
ou context foi realizada.

## Medição da fonte anterior à classificação

- Vanilla ratificado `a51e02804`, `loading/csv.rs:138-157`: todo erro CSV
  passa por `LoadError::text`, com causa escolhida pela variante do parser e
  posição pelo offset do parser, com fallback ordinal/coluna inicial.
- Vanilla `diag.rs:851-855`: a validade UTF-8 do buffer inteiro decide entre
  apresentação textual e binária. Não depende de `ErrorKind::Utf8`. Portanto
  um erro UnequalLengths anterior a um byte inválido posterior também cai
  na apresentação binária, sem trocar sua causa vencedora.
- Vanilla `diag.rs:895-923`: Path binário acrescenta ` in <path>:L:C` dentro
  do envelope da causa e usa o span do argumento carregado. Project usa vpath
  sem slash inicial; Package usa `PackageSpec` seguido da vpath com slash.
  O caminho permanece na mensagem mesmo quando a posição não é disponível.
- Vanilla `diag.rs:858-873`: Path textual válido usa `FileId` e range no
  arquivo externo; não corresponde ao ramo de span do argumento.
- Cristalino `contracts/world.rs:49-65`: `resolve_path` retorna RootedPath,
  `read_path` retorna bytes, `include_path` inclui Source. O contrato não
  oferece operação RootedPath → FileId isolada.
- Cristalino `entities/path.rs:79-93`: RootedPath contém root/vpath sem ID.
  `compiler/stdlib/foundations/path.rs:15-34` preserva Path, resolve Str uma
  vez e retorna `(RootedPath, bytes)`. Seu L0 confirma essa obrigação.
- Cristalino `loading.rs:1117-1125`: helper local reduz RootedPath a uma
  string vpath, perdendo a raiz de pacote. `loading.rs:1374-1375` ainda
  descarta até essa string e chama o decoder puro.
- Cristalino `loading.rs:948-1058`: mapper mantém acesso à variante, ordinal
  e posição do parser; helper privado já distingue texto/binário pela validade
  integral. `loading.rs:1357-1372` recupera o span causal de Bytes do Args.
- `03_infra/src/world.rs:744-764`: read_path registra internamente um ID e
  devolve bytes; include_path registra e pede Source, convertendo falhas em
  mensagem de include. Conhecer essa implementação não amplia o contrato do
  trait nem garante include em um World que suporta somente leitura binária.

## Parecer prévio e fronteiras

É inferência fundamentada que o recorte **CSV Path/Str cujo buffer inteiro é
UTF-8 inválido, após qualquer falha vencedora de parsing** cabe exclusivamente
no owner loading. RootedPath já fornece o caminho virtual completo necessário,
Args já fornece o span do argumento e o mapper já dispõe da posição antes da
formatação. Não há necessidade identificada de FileId externo neste recorte.

Classe preliminar ADR-0127: correção interna de paridade, fluxo contínuo com
L0 primeiro, RED→GREEN e revalidação. Não adiciona tipo aceito, campo, método,
assinatura pública, flag, fase ou novo comportamento intencional de produto.
O novo L0 deve substituir expressamente a preservação histórica de Path/Str
somente nesta partição. A aprovação anterior P1313 não autoriza mudanças
adicionais em casts ou transportes.

Texto UTF-8 válido permanece uma dívida distinta. Não usar include_path para
fabricar identidade externa: além de trazer contrato de Source/include, tal
rota pode exigir capacidade não implementada por Worlds binários e introduzir
falha antes inexistente. Alteração de World, RootedPath, Source ou assinatura
pública para solucionar essa dívida tem owners próprios e reabre ADR-0127.
Este parecer não determina a arquitetura dessa etapa futura.

Não selecionar o recorte por mensagem, extensão, fixture ou variante Utf8.
Não antecipar validação UTF-8 ao parser para escolher outra causa. Não montar
posição extraindo números de texto formatado. Não re-resolver Path nem fazer
duas leituras. O helper compartilhado de path já retorna os dados necessários;
qualquer refactor local deve preservar mensagens de I/O e outros loaders.

## Critérios para revisão do L0 e congelamento pré-patch

O contrato precisa declarar caminho Project/Package, origem causal/With/Args,
detached legítimo, ausência de posição, ordem externa de validação e o caso
UnequalLengths vencedor antes de byte inválido posterior. Controles devem
incluir UTF-8 válido (diagnóstico legado integral), Bytes (P1318 integral),
valores válidos, opções/casts/I/O, decode_csv público e outros loaders.

Testemunhas de paths normalizados, Unicode e root de pacote distinguem vpath
real de string lexical/PathBuf e impedem perda de PackageSpec. Com World
simulado, read_path deve bastar; include_path/source podem ser proibidos.
Opção inválida deve continuar falhando antes de leitura. CRLF e prefixo sem
LF distinguem a conversão binária de uma conversão textual intuitiva.

Parsing com excesso continua a precedência legada. Se receber a nova
apresentação por pertencer à partição binária, o efeito deve ser declarado
normativo, com comparação baseline própria, sem rotular como paridade com
o erro vanilla de excesso.

Mensagem e origem diagnósticas são observáveis de linguagem pela exceção
ADR-0108. Igualdade de estruturas Rust ou caches não serve como aceitação.
Necessidade de outro owner, falha de origem conservada no vanilla, nova leitura
obrigatória ou divergência no formato Package refuta a inferência local.

Parecer prévio favorável ao recorte; revisão do novo L0 e da medição ainda
pendente. Nenhuma implementação foi avaliada ou autorizada por este documento.
