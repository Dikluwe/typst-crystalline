# Residual da stdlib sem imports

Overlay de laboratório; não altera o produto nem L0. `apply.py` só aceita cópias
separadas sob `/dev/shm/tekt-stdlib-residual-20261105`, recusando o baseline.
O binário diagnóstico oferece quatro políticas, escolhidas em L3 por
`TEKT_LAB_STDLIB_POLICY` (um caractere):

|Valor|Root|Imports|
|---|---|---|
|0|Scope original, sem snapshot|Builder por arquivo|
|1|Snapshot antecipado|Builder por arquivo (controle de captura)|
|2|Snapshot antecipado|Instâncias por arquivo com guard|
|3|Scope original, sem snapshot|Primeiro import cria template; próximos usam guard|

A política 3 mantém o slot vazio quando não há imports. Com inputs/features
estáveis, são dois builders por entrada (root + primeiro import), sem cache
entre compilações. Cada arquivo conserva identidades novas e aliases internos;
o guard exige pares de inputs na mesma ordem e todas as features. Um mismatch
cria um template para aquele subtree, sem substituir o seed do caller.
O helper é limitado à stdlib pinada, não é clonagem geral de valores avaliados.

O método `World::__lab_stdlib_policy`, os portais públicos de diagnóstico e as
sondas NÃO são propostas de contratos produtivos. `--mutation` oferece quatro
falhas deliberadas; erro de compilação não conta como rejeição por teste.

`measure.py --variant bas=/CAMINHO/typst:- --variant p00=/CAMINHO/diagnostico:0
--variant p03=/CAMINHO/diagnostico:3 ...` usa as mesmas fixtures/ciclos do
experimento anterior, rotacionando os variantes. Nomes de igual comprimento
ajudam a manter caminhos comparáveis. Use `--fixtures` apontando para
`../stdlib-reuse/tests/fixtures`, `--fixture imports_00.typ` e diretório novo
em RAM. Contadores, builds, profilers e extração de PDF ficam fora dos relógios.

`--probe` habilita `TEKT_LAB_PHASE_FILE` por processo. Sem essa variável, não há
clocks ou saída das sondas, mas o check de opt-in permanece e requer calibração.
As faixas são inclusivas e se sobrepõem: não some fontes/eval/paged/export como
partição. `phase_harness.rs` compara APIs públicas com Worlds independentes:
configuração, Source, primeiro book e eval/drop em uma; PDF padrão completo em
outra. A segunda repete eval/fontes; seus valores não se subtraem da CLI.

Compile `tests/residual_observables.rs` contra as mesmas rlibs release com
`--test -C opt-level=s`, externs core/comemo/ecow e dependências do target.
Baseline: sem cfg diagnóstico. Candidato: `--cfg residual_candidate` e
`--cfg stdlib_reuse_candidate`. Somente a bancada de alocações acrescenta
`--cfg residual_alloc`; execute o teste
`cold_allocations::lazy_no_import_root_has_no_extra_allocation_over_legacy`.
Nunca use os tempos dessa variante. O benchmark ignorado de eval usa Sources
pré-parseadas e recebe a política pela mesma variável de ambiente.

`apply_static.py` instala o candidato nominal: builders originais no root,
slot opcional vazio no EvalContext e política 3 constante nos imports. Não
instala o método World, seletor, portais ou sondas. Código privado não usado do
helper permanece no arquivo de lab; o build próprio precisa confirmar sua
eliminação. Revalide os controles públicos e o pipeline no ELF nominal.

Para comparar alocações nominal/baseline, compile com `residual_alloc` e
`residual_nominal`, sem `residual_candidate`, e rode somente
`cold_allocations::cold_boundary_child` com `TEKT_RESIDUAL_COLD_CHILD=3`.
O comparador de quatro políticas continua exclusivo do diagnóstico. O filtro
`TEKT_RESIDUAL_ONLY_ZERO=1` limita a microbancada ao controle 0/flat.

O scope `source_access_inclusive` mede o acesso via World::source, que pode ser
um hit de cache: não o apresente como tempo de parse. O harness público também
não separa exclusivamente parse da aquisição/configuração. Para investigação
submilissegundo, use os tempos sem sondas; a calibração faz parte do resultado.
