# Relatório final da etapa — TS1 e igualdade Styled

**Etapa encerrada com as regressões observadas de TS1 corrigidas.** O projeto inteiro continua aberto: há 124 V16 e frentes de paridade independentes no inventário. Não houve commit/push do Typst, merge ou release.

## O que foi entregue

[Checkpoint TS1](../text-size-length-2026-10-17/relatorio.md): Style::Size(Pt)→Size(Length) e StyleDelta.size:Option<f64>→Option<Length>, conforme aprovação humana de manifesto 771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba. TextStyle.size permanece Pt resolvido. Constructor/set conservam abs+em, busca posicional por tipo, preferência nomeada, corpo None/Str/Content, diagnóstico e ocorrência causal. Captura/collapse fazem composição afim; Raw/math usam comprimentos absolutos já resolvidos. Heading afim e filhos relativos foram corrigidos em layout, sem mover fase. Foram 20 consumers, incluindo o adapter de Heading exigido pelos testes, e 20 L0s mais o núcleo compartilhado.

A validação TS1 expôs duas regressões de igualdade (relative-vs-zero e affine-vs-absolute-part, true→false). Este lote separado as fecha: somente `01_core/src/compiler/eval/operators/equality.rs` e seu L0 mudaram desde o checkpoint. Após morph_canon existente, o operador compara uma projeção temporária que conserva Styled/child, coalesce Styled adjacente e ignora seu mapa; todos os outros nós permanecem completos. A projeção não entra no render, em Value/Engine, Hash, ObservationEq ou terminação. **PartialEq Rust e morph_canon não mudaram.** Wrapper versus plain, Strong/Emph e corpos distintos continuam distintos. Eq/Neq, arrays/dicts e pertença usam a mesma relação.

Foi correção interna de paridade em fluxo contínuo ADR-0127, com L0 antes de teste/código; nenhuma nova API pública, default ou fase. A referência pinada foundations/content/mod.rs:376–385,779–782 mede coalescência e igualdade pelo child. Isso não é igualdade dos valores de tamanho declarados nem apagamento indiscriminado de wrappers.

## Testes e resultados

TS1 teve seis RED iniciais e quatro adicionais de Heading, 14 testes novos GREEN e 8810 testes aprovados no checkpoint. Igualdade teve cinco RED funcionais e um grupo negativo já aprovado; depois os **seis testes novos passaram**. A suíte final completa tem **8816 aprovados, zero falhas e sete ignorados**.

**24 pares bilaterais de igualdade coincidem agora: 17 divergências reparadas e sete controles preservados.** Incluem as duas regressões TS1, tamanho relativo/resolvido, cores/lang, wrappers adjacentes, filhos nominais, containers, in/not-in, != e NaN de estilo, sem tornar Float NaN reflexivo. O primeiro corpus tinha chamadas aninhadas como texto literal e calc.nan ausente; foi corrigido antes de especificar/implementar. Não conta como RED funcional. Um import de teste introduzido mecanicamente foi removido antes dos gates finais.

O replay dos **105 pares textuais TS1** preserva 102 casos na geometria/saídas medidas e muda somente três resultados booleanos false→true, coincidentes com o pin. Os **50 pares Par/Box** mantêm a geometria anterior dentro de 0.002pt. Os **105 pares RG1** preservam diagnósticos/efeitos, estrutura HTML e linhas coloridas SVG. Oito pares de cor sólida já foram validados no checkpoint em vetores de PDF, comparando a paleta real (#ff4136), sem presumir vermelho RGB puro.

Build workspace, fmt, Clippy workspace/all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices HTML, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. SARIF final: **{'V16:error': 124, 'V19:note': 1069, 'V20:note': 865}**. Nenhum erro/warning novo; diagnósticos fora do consumer de igualdade são idênticos ao checkpoint e V16 por consumer permanece. O gate global continua exit 1 pelos 124 V16 conhecidos; não foi relaxado, e não houve supressão genérica de V19.

## RAM, API e proveniência

Teto rígido **12 GiB**, MemoryHigh 10 GiB, swap zero, Cargo jobs=1 e comandos pesados sequenciais. Pico deste lote **10742411264 bytes (10.005 GiB)**; maior pico dos dois lotes **10742411264 bytes (10.005 GiB)**. Eventos deste lote: {'high': 1414, 'max': 0, 'oom': 0, 'oom_kill': 0}. Sem OOM/OOM-kill. Memória de cgroup inclui cache, não é RSS nem prova de otimização.

StyleDelta/Styles medidos 336bytes, antes 328; Style permanece 32. O custo aprovado é +8bytes por campo de tamanho. Chamadores Rust devem migrar Size(Pt(v)) para Size(Length::pt(v)) e size:Some(v) para Some(Length::pt(v)). A sonda externa da API do checkpoint passou. Este lote de igualdade não mudou layout de tipos.

Os probes não finitos iniciais de TS1 usavam namespaces ausentes e não provam admissão desses valores. O lote final usa float("nan")/float("inf"); códigos medidos: `{'zero': {'tekt': 0, 'vanilla': 0}, 'negative': {'tekt': 0, 'vanilla': 0}, 'nan': {'tekt': 0, 'vanilla': 0}, 'infinity': {'tekt': 0, 'vanilla': 0}, 'negative-infinity': {'tekt': 0, 'vanilla': 0}, 'nan-em': {'tekt': 0, 'vanilla': 0}, 'infinity-em': {'tekt': 0, 'vanilla': 0}}`. A certificação é de admissão/repr, não de render NaN/infinito. Zero/negativos finitos também têm coletas de render no checkpoint; não foi inventada uma restrição de positividade.

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. before/final-state.json.gz, RED/GREEN, validation-result.json, recursos e corpora registram UTC, status/diffstat, hashes de todas as fontes/L0/configuração e de binários/artefatos, comandos e saídas. Vanilla **9f2b6e8715237cb086899a42873660fe744622e8**, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter **12797073972e375d9ab078cf6750c8b30b71e2b9**, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2. A branch autorizada do linter já estava publicada e seu pin CI foi preservado.

Os manifestos públicos TS1/RG1 foram conferidos integralmente e preservados. RG1 continua aguardando confirmação; ao materializar seus candidatos, rebasear apply/tests sobre TS1 e esta correção, sem sobrescrever a árvore. Nenhum contrato RG1 foi implementado aqui.

## Ordem seguinte e limites

[Inventário atualizado](inventario-v16-restante.md): 124 V16 em 95 consumers. Retomar primeiro os oito casos já triados de type_fields/collections/figure_image/casting, medindo antes e sem alterar API sem lista/revisão. Figure/kind e casts relativos têm limitações próprias; não trocar por grupos silenciosos.

Permanecem Paint Gradient/Tiling textual, seletor show text, defaults de Heading substituído por função, scripts, presença de constructores custom-only fora da projeção aqui medida, Box1fr, SVG multipágina e literal inteiro mínimo. O lote não certifica todo Content nem toda a paridade. As diferenças de scripts são reproduzidas no controle absoluto e relativo de cada backend. [Pendências](pendencias.json) conserva a lista; as duas regressões de igualdade que impediam a entrega desta etapa estão resolvidas.

[Patch desde TS1](patch-since-TS1.diff) · [Checkpoint anterior](../text-size-length-2026-10-17/final-state.json.gz).
