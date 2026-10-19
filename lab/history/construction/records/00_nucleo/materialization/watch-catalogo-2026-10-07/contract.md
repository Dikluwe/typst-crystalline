# Contrato revisável — retenção validada de fontes no watch

Status: aguardando ratificação concreta. Não é Prompt L0: coordena os sete
candidatos L0 no manifesto. Evidência anterior à decisão:
00_nucleo/diagnosticos/watch-validacao-2026-10-07.md e seus receipts; contagem/corpus
baseline em 00_nucleo/diagnosticos/watch-retencao-2026-10-07.md.

A validação isolada final mediu 49,26 ms contra 100,04 ms da aquisição completa;
a série inicial mediu 65,56 contra 128,62 ms. A variação impede certificado estrito.
Treze controles de chave passaram. Nenhum ganho de watch implementado/prometido.

## Mudança autorizável

Adicionar, somente em L3:

```rust
// typst_infra::world (tipo opaco, sem campos públicos)
pub struct FontCatalogSession { /* private */ }
impl FontCatalogSession { pub fn new() -> Self; }

impl SystemWorld {
    pub fn with_font_catalog_session(
        self,
        session: Arc<FontCatalogSession>,
        font_paths: &[PathBuf],
    ) -> Self;
}
```

Watch cria uma sessão por execução e continua criando um World por recompilação.
A sessão guarda no máximo uma geração para futuro reuso, Arc do book/slots
pareados. Configurar builder não faz I/O. Na primeira demanda de fontes, repetir
a descoberta atual do fontdb/projeto, comparar ordem/path/face e identidades Unix
(dev/inode/len/mode/mtime/ctime com nanos), rechecar e reutilizar apenas geração
válida. Novo catálogo usa a descoberta já preparada e o extrator vigente da
prioridade 3, sem rediscover desnecessário. Mudança/recusa/instabilidade cai para
rota fresca e não cacheia fallback. Binary continua excluído; SharedFile recusa
retenção e conserva o caminho fresco sobre bytes retidos. Sistemas não Unix
ficam frescos. Limite de elegibilidade: 16384 descritores/1 MiB de paths.

Sources, erros negativos, read_cache, FileIds/deps, inputs, plugins/downloader,
relógio, métricas, shaping e eval/layout continuam com suas durações atuais.
Coverage materializada no book segue sua geração; coverage_cache ausente é
fresco por World. Sem global, novo unsafe, novo trait ou mudança de pipeline.
Compile comum continua fresco, sem nova flag. Fontes sozinhas não criam evento
novo: alterações são vistas na próxima recompilação. Status/diagnósticos,
arm/publicação atômica, HTTP/artefato anterior, evict e polling permanecem.

## Fronteira de ratificação

ADR-0127 §2.1: duas assinaturas públicas novas e tipo público opaco.
ADR-0127 §2.2: aquisição com sessão passa a caminho padrão do watch.
Confirmar este contrato e o manifesto antes de qualquer instalação de L0 ou
código produtivo. Candidatos L0 e Núcleo, hashes e bases estão no manifesto;
aprovação anterior de prioridade 3 não ratifica este novo contrato.

## Execução após confirmação

1. Verificar novamente bases dos owners e pins existentes; instalar candidatos.
2. Testes RED→GREEN inline nos owners novos e builder: reuso/renovação, instabilidade,
   SharedFile, limites, erro/poison, concorrência, negativo→válido e fontes alteradas.
3. Implementar somente sete consumers, mantendo seletivo/coverage e outros owners.
4. Resselo local depois do grafo 1:1 válido, build e crystalline-lint completo com
   zero erros; testes pertinentes e suite necessária, sem misturar warning com erro.
5. Congelar anterior96f22 e nova receita/HEAD/dirty/UTC/fonts/binários; comparar
   main/import/read/PNG, falhas/recuperação, adição/remoção/substituição de fontes,
   seleção/texto/páginas/fontes/caixas/tinta. Preservar negativos independentes.
6. Medir startup e ciclos reais separadamente, pelo menos três rodadas pareadas
   no mesmo processo por watcher, CPU/latência/RSS e contagens fora dos timings.
   Sem ganho repetível ou com regressão, registrar negativo e reavaliar/reverter
   o caminho otimizado; não fechar D10 apenas por testes ou custo isolado menor.
