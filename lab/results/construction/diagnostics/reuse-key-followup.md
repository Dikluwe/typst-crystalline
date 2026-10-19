# Reutilização confiável: fontes e chaves de shaping

Esta continuação transforma um risco do estudo anterior em um contraexemplo real: a chave de shaping arredonda pesos distintos para o mesmo texto e reutiliza um avanço incorreto. Também demonstra, em um World limitado ao diagnóstico, que fontes podem sobreviver à troca do snapshot de sources. Nenhum código produtivo foi alterado e nenhum ganho de desempenho foi medido aqui.

## Evidência antes da decisão

O consumer `03_infra/src/shaper/cache.rs:34` constrói a chave com variações formatadas em quatro casas decimais (`:42`). `03_infra/src/shaper/shaping.rs:229` consulta essa chave; quando há miss, aplica as coordenadas originais à face (`:238`) e grava os glifos (`:324`). Assim, o shaper recebe valores mais precisos do que a chave distingue. O L0 `00_nucleo/prompts/infra/shaper/cache.md` atualmente exige essa formatação; o consumer segue esse contrato. O problema exige corrigir o contrato junto com a implementação em uma tarefa posterior, sem tratar o L0 atual como autorização para outra precisão.

A sonda usa somente `shape_document`, `SystemWorld`, `FontSlot` e entidades públicas existentes. Não chama nem publica a chave privada. Primeiro procura mudanças no avanço de glifos com ttf-parser, depois exige diferença no resultado fresco do shaper público. Só então compara AB e BA no mesmo documento. Portanto, a descoberta não depende apenas de uma colisão textual hipotética.

| Fonte / texto | Peso A (f32 promovido para exibição) | Peso B | Chave do eixo | Avanço fresco A/B | Documento AB | Documento BA |
|---|---|---|---|---|---|---|
| Ubuntu Sans / A | 104.54103088378906 | 104.5410385131836 | 104.5410 | 617 / 618 | 617 / 617 | 618 / 618 |
| Cantarell / x | 113.05543518066406 | 113.0554428100586 | 113.0554 | 446 / 447 | 446 / 446 | 447 / 447 |

Avanços estão em unidades da fonte. Coordenadas são f32 adjacentes, com bits preservados em `results.json`. As consultas frescas têm caches documentais independentes; as consultas AB/BA compartilham o cache de um documento. Mantêm texto, família explícita, face 0, tamanho 12pt e demais campos do estilo iguais. O primeiro resultado vence nas duas ordens. A diferença é pequena nesses exemplos, mas prova que esta chave não é suficiente para confiar na equivalência dos resultados. Não demonstra frequência do problema em documentos reais, diferença visual de PDF ou impacto no tempo total.

Os mesmos resultados foram reproduzidos em duas novas execuções do executável arquivado, além da execução inicial. Não foram coletadas amostras de tempo. Os timestamps descrevem proveniência, não desempenho.

## Fontes persistentes e sources renovados

O protótipo diagnóstico `reuse-key-followup/src/main.rs:61` possui um pool Arc com um SystemWorld sintético de fontes e o Font adquirido. O catálogo continua owned por esse World, sem necessidade de tornar FontBook clonável. Cada Snapshot cria outro SystemWorld para arquivos e delega book/font ao pool. Métodos de main/source/file/library/today são implementados; os testes exercitam main/source/font/book. Os demais métodos conservam defaults e não constituem um World completo de CLI.

Controle observado:

- O primeiro snapshot lê `First snapshot`.
- Após editar o arquivo, o novo snapshot lê `Edited snapshot` com o mesmo pool.
- Os Font handles vivos compartilham o mesmo buffer, enquanto o snapshot antigo continua lendo seu source original.
- Uma substituição explícita troca catálogo e Font como um par: Ubuntu Sans por Cantarell no slot 0. O snapshot antigo conserva seu par válido.

Isso demonstra separação de ownership usando interfaces já existentes. Não implementa descoberta automática de alterações de fontes, catálogo serializado em disco, fallback, cobertura, pacotes, plugins, arquivos auxiliares, watch ou persistência de resultados entre processos. Não mede o custo de construir snapshots nem o de descobrir fontes. O pool é materializado explicitamente com uma fonte; não representa a política lazy de um catálogo de sistema inteiro.

A leitura anterior do vanilla pinado mostra retenção em memória de slots/fontes durante watch e renovação do estado de arquivos. O protótipo confirma uma possibilidade de separação no Tekt; não testa equivalência de todos os observáveis com o vanilla. O vanilla não foi executado nesta sonda. A proveniência do pin é mantida pelas verificações dos datasets anteriores, sem transformar isso em prova de paridade destes casos.

## Consequências para evitar retrabalho

A evidência favorece conservar recursos de fontes dentro de uma sessão com identidade e invalidação explícitas, renovando snapshots de arquivos. Conservar o World inteiro sem renovação já havia preservado sources antigos no estudo anterior. Persistir indiscriminadamente o cache de shaping agravaria o risco encontrado agora: duração maior não torna uma chave incompleta correta.

O owner da chave continua `infra/shaper/cache`; aplicação das variações e consumo do resultado pertencem a `infra/shaper/shaping`. O ciclo de aquisição/fontes pertence a L3; L4 apenas compõe os componentes segundo seus contratos. Uma futura materialização precisa ler/atualizar os L0s afetados e classificar o gate ADR-0127. Nenhuma API nova é necessária para este diagnóstico.

Experiências seguintes que podem confirmar ou refutar propostas:

1. Corrigir a identidade da chave e usar estes dois casos como regressões independentes: resultados mistos devem coincidir com resultados frescos em ambas as ordens. Testar também campos de tracking e script separadamente; eles não foram inocentados por esta sonda.
2. Para uma sessão real de fontes, testar alteração/remoção/reaparecimento de arquivos, ordem do catálogo e mudança de opções. Comparar cada nova geração com um World independente; todos os pares book/slot/font precisam corresponder. O protótipo só verifica troca manual entre dois universos.
3. Depois de estabelecer correção, medir sessões frias e recompilações quentes do mesmo workload, com inputs/fontes/opções equivalentes e exclusão de concorrência. Só essa experiência pode atribuir uma economia à retenção de fontes. Os tempos exploratórios anteriores não autorizam estimar esse ganho.

## Reprodução e proveniência

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitada. `status.txt`, `diff-stat.txt` e `incoming.patch` preservam o estado exato antes da execução. A patch tracked permaneceu idêntica; as adições desta tarefa ficam apenas neste diagnóstico e neste relatório. As falhas preparatórias de compilação do protótipo foram arquivadas em `preparation/`; não geraram resultados aceitos.

Execução inicial: UTC `2026-09-27T16:08:58.568291+00:00` até `2026-09-27T16:08:58.573682+00:00`. Rust 1.92.0, release opt-level 3, debug_assertions=false, overflow_checks=false, test=false. RUSTFLAGS e CARGO_ENCODED_RUSTFLAGS vazios; overrides CARGO_PROFILE/TYPST/CRYSTALLINE removidos; um job de build. Máquina em `machine.json`, comandos completos/perfil/toolchain em `provenance.json`. Dependências resolvidas offline e recompilação com --locked. Executável SHA-256 `f28bb650a1c383bf2b94c7940b461f822b58e288e6b267fb114624d4025b5cff`.

Fingerprint dos consumers/fontes previamente auditados e fonte vanilla: `9d29d3f8b1a0223b55eff92b955b4cd0735d205cd4c49970825ae2047bc44fe2`, igual antes/depois. As duas fixtures adicionais têm hashes próprios em `inputs.json` e são verificadas pelo probe e pela auditoria final. Dados/fontes consumidores anteriores, relatórios selados e binários anteriores permanecem íntegros. Licenças das fixtures: `03_infra/fixtures/fonts/LICENSE-UbuntuSans.txt` e `LICENSE-Cantarell.txt`.

Protocolo e busca delimitada em `reuse-key-followup/README.md`; código em `src/main.rs`; resultados em `results.json` e `repetition-{1,2}.json`. Para verificar sem alterar evidência:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/reuse-key-followup/verification/verify.py
```

O runner rejeita um diretório que já contém results.json. Para reconstrução, preserve o conjunto selado e use uma cópia nova do crate/protocolo, ajustando seus caminhos locais. A busca percorre wght de 100 a 800 em passos 0.5 e termina no primeiro caso discriminante por fonte; não é uma auditoria exaustiva do espaço de estilos.
