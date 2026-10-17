# Prioridade 6, lote A — fontes sintáticas entre edições de watch

Proposta pendente do gate ADR-0127 §2.1. Não autoriza código antes da ratificação deste manifesto. Este passo não é L0: os candidatos abaixo especificam owners exclusivos e serão instalados nos caminhos canônicos somente após confirmação.

## Evidência antes da decisão

`00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/` guarda HEAD/árvore/UTC/binários e medições novas. `count.json` registra duas Sources construídas e uma entrada de import a cada ciclo, mesmo quando o import não muda. Source::new chama parse e numberize; World novo preserva cache apenas dentro de cada ciclo. `compiler/eval/modules.rs:44-115` executa o import sem memoização. `contracts/world.rs` mantém World puro, sem rastreamento integrado. ADR-0005 prevê separação B3; ADR-0030 situa comemo em L3. Vanilla eval tem memoize com World tracked; esse mecanismo não pode ser transplantado para chamadas não rastreadas do Tekt.

Watch medido: import de 30.000 iterações custa aproximadamente 68 ms CPU/edição no Tekt; 4.000 declarações, 60 ms; import mínimo, 53 ms. Eval_ms inclui recursos demandados pela avaliação. Essas diferenças entre fixtures não isolam causalmente parsing ou cálculo. A latência inclui polling e não é comparável à CPU. A sonda independente de Source quantifica somente parsing vs comparação completa/clone; não é prova de ganho do produto.

## Decisão proposta e limites

Primeiro preservar Sources sintáticas imutáveis, com leitura e validação completa por ciclo, identidade de arquivo corrente e retenção limitada. É fundação incremental, não memoização de Module, funções, efeitos ou documento paginado. A avaliação e o layout ainda executam em cada ciclo; o lote B exige contrato próprio de dependências/efeitos para memoização de imports, e C exige observação causal de conteúdo/estilos/recursos/páginas para layout. Não considerar a prioridade 6 inteira encerrada com este lote.

## Superfície pública nova

No namespace existente `typst_infra::world`:

```rust
pub struct SourceSession { /* estado privado L3 */ }
impl SourceSession { pub fn new() -> Self; }
impl SystemWorld {
    pub fn new_with_source_session(
        root: impl Into<PathBuf>,
        main_path: impl AsRef<Path>,
        session: Arc<SourceSession>,
    ) -> Result<Self, SystemWorldError>;
}
```

`new()` existente e os constructors de stdin/expressão preservam contratos e caminhos. Nenhum campo/método de World, Source, Engine ou trait de L1 muda. Não criar TrackedWorld global ou memoizar contra Source::Eq/hash u64. A sessão é opt-in na API e mantida por uma chamada watch; a CLI não ganha flag.

## Invalidação e isolamento

Cada World conserva mapas/IDs/alocação/read bytes/dependências/coverage/input/plugin/date novos. A sessão não publica nem restaura IDs: só pode reutilizar Source para o FileId já atribuído pelo World corrente, mesmo caminho canônico físico e texto UTF-8 integral exatamente igual. Igualdade de mtime/size ou hash não autoriza hit. Arquivo deve ser relido via política vigente antes de comparar; erros de acesso/UTF-8 não recebem cache inter-World. Syntax errors/warnings na CST seguem publicação normal em cada avaliação; erros de leitura continuam OnceLock apenas no World corrente.

Sem Location, Module, Func, Sink, Route, Introspector, Frame, fontes, bytes binários, pacote/manifest ou clock na sessão. Source Arc mantém seu CST/span sem mutação. Resolver imports/read sempre pelo World novo; dependências atuais continuam registradas até num hit. Excluir transientes/synthetic/code/math; permitir apenas Source markup física do owner cache. FontCatalogSession permanece independente e sob seu contrato já ratificado.

## Retenção proposta

LRU por sessão com no máximo 8 entradas, 1 MiB de texto UTF-8 agregado e 512 KiB por entrada. Somente sucessos Source markup; entrada maior segue caminho normal sem retenção. Orçamento de texto não é limite de RSS/CST. Medir RSS e discutir novo orçamento se custo superar benefício. Mutex por instância em L3; sem globals, unsafe, disco persistente ou relógio para LRU. Mesmo key com texto diferente substitui entrada; retorno ao texto antigo só reutiliza se ainda presente e completamente validado. Evicção não altera semântica. Concorrência pode repetir parsing, mas nunca misturar texto/id/caminho ou substituir o resultado observado por outro World.

## Integração e gate de aceitação

Cinco owners, um núcleo compartilhado. world controla constructors/estado; cache lê/valida e entrega SourceSession; source_session decide retenção e reuso sintático; compile escolhe constructor antes da leitura eager; watch mantém duas sessões fora do loop. Preserve staging/arm/publicação, erros, recuperação, HTTP, polling e evict(10). Nenhuma transferência eval↔layout.

Após ratificação: instalar candidatos e pins; testes primeiro RED por Source não compartilhada no segundo World; também ID diferente, caminho diferente, texto alterado (inclusive mesmo mtime/size), Unicode/UTF-8 inválido, sintaxe/warnings, arquivo removido/recriado e limites/LRU/concor­rência. Semântica: imports relativos/alias/include/read, FileId alterado por ordem de imports, errors/carets, efeitos/query, assets e render preservados. GDB: main edit deve continuar construindo Source nova, import inalterado deve deixar de parsear; import edit deve parsear novamente. Comparar binário congelado P5/final com três rodadas pareadas, CPU/latência/RSS/startup separados, controles de arquivos pequenos e erro/recuperação. Build, suítes pertinentes e lint com zero erros, selos apenas nos owners. Retirar o caminho de produção se não houver benefício repetido ou se houver observável alterado. Não extrapolar a sonda clone para ganho CLI.

## Ratificação solicitada

Confirmar o manifesto SHA-256 e os candidatos listados, incluindo tipo/constructor novos e orçamento de retenção, autoriza somente lote A. Memoização de avaliação e layout dos lotes B/C continua fora desse gate.
