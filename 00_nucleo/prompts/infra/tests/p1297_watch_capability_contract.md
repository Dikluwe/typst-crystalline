# Prompt L0 — contrato externo P1297/R1 da capacidade armada de watch
Hash do Código: 3c475199

**Camada:** L3 (integration test externo)
**Ficheiro alvo exclusivo:** `03_infra/tests/p1297_watch_capability_contract.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129
**Origem causal:** P1297/R1 confirmado; gate humano `Continue` registrado no
receipt SHA-256
`11777b20d0fc62256077339c35b86c559ba90cce167da2aee2ca081a9bd446e2`

## Propriedade, regime e independência

Este Prompt possui exclusivamente o integration test externo acima. Não
legitima `03_infra/src/watch.rs`, `04_wiring/src/main.rs` nem a suíte CLI. O
regime é o protocolo Tekt completo: P2-R1 autora contrato e oráculos a partir
dos L0s confirmados, baseline pré-R1 e evidência predecessora, sem ler nem
adaptar-se a implementação candidata R1, que não existe nesta fase.

O workspace é compartilhado e não fornece isolamento técnico de leitura. A
independência alegada limita-se à segregação causal por entradas, capacidades,
ordem e allowlist de escrita do manifesto P1297, SHA-256
`5bf3f29e353403006b90971e05f786ad2548648c053b9db59f879841aecdf322`.
`Unknown` nunca é sucesso nem recebe crédito; qualquer `Unknown` inesperado
bloqueia R1.

## Medição anterior à decisão

No baseline R1 confirmado, HEAD
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`,
`03_infra/src/watch.rs` SHA-256
`0232f1baa3b06bde809941938ea0633982a78ec066ce8e38466f346e63403a35`
ainda expõe `WatchSnapshot`, `snapshot`, `wait_for_change_since`,
`wait_for_change`, `commit_output` e `discard_output`; não existem
`ArmedWatch`, `arm`, `publish` ou `abandon`. Em
`04_wiring/src/main.rs:95-122`, L4 chama captura e finalização como operações
públicas independentes. Portanto o RED esperado é de compilação por ausência
da API R1, não por timeout, ambiente ou comportamento probabilístico.

P1296 demonstrou que observar a remoção de staging depois do fato não prova a
ordem: MO1 `discard -> snapshot` sobreviveu com score `2/3`, e a revisão FIFO
regrediu o controle positivo. A nova testemunha sobrepõe deliberadamente o
path observado ao staging. Se a captura ocorreu no `arm`, o snapshot contém
`Some(fingerprint)` e a finalização produz `None`; se a captura for lazy após
a finalização, contém `None` e a espera não pode retornar sem estímulo novo.

Classificação ADR-0107/0108: tipos, campos, algoritmo de fingerprint, JSON e
forma named/tuple são mecânica. Aqui a transição de filesystem e a superfície
pública são os observáveis causais do contrato. Inferência: a sobreposição
controlada discrimina captura anterior de captura lazy sem observar o
intervalo entre chamadas. Refutador: uma implementação correta que não consiga
preservar os positivos, algum R1M1–R1M4 sobreviver, rustdoc omitir superfície
nociva, ou a prova depender de sleep, carga, retry ou polling do harness.

## Oráculos funcionais determinísticos

Cada caso usa fixture temporária exclusiva e limpeza RAII. Toda transição é
concluída antes de iniciar a thread que chama `wait_for_change_since`; o
timeout de dois segundos limita apenas a falha do runner.

### A — abandono depois do armamento

1. Criar um ficheiro válido.
2. Usar esse mesmo path simultaneamente como observado e staging.
3. Chamar `arm([path])` enquanto o ficheiro existe.
4. Consumir a capacidade com `abandon(path)` e exigir remoção do staging.
5. Entregar o snapshot retornado à espera e exigir retorno.

O caso prova `Some -> None`, mata R1M1 e também R1M4. Captura lazy depois da
remoção produz `None -> None` e deve falhar pelo timeout, sem segunda escrita.

### B — publicação depois do armamento

1. Criar staging válido e destino válido distinto.
2. Observar apenas o staging e chamar `arm([staging])` enquanto ele existe.
3. Consumir a capacidade com `publish(staging, destination)`.
4. Exigir desaparecimento do staging, conteúdo novo no destino e retorno da
   espera com o snapshot devolvido.

O caso prova `Some -> None` no staging e preserva rename atômico. Mata R1M2 e
R1M4; captura lazy depois do rename deve ficar bloqueada até o timeout.

### C — abandono preserva o destino

Criar staging e destino com bytes distintos, armar o staging, abandonar e
exigir staging ausente com destino byte a byte inalterado. A falha best-effort
de cleanup não pode autorizar remoção ou substituição do destino válido.

### D — falha de publicação preserva causa e faz cleanup best-effort

Dois controles de fronteira usam destino sob parent inexistente, produzindo
falha de rename antes de qualquer substituição:

1. com staging-file removível, medir previamente a classe e o código OS do
   erro de rename, chamar `publish`, exigir o mesmo erro original e staging
   removido;
2. com staging-diretório, medir previamente o erro de rename e também o erro
   distinto de `remove_file`, chamar `publish`, exigir que o erro devolvido
   continue sendo o de rename, mesmo quando o cleanup best-effort falha.

O oráculo não fixa wording de plataforma. Compara `ErrorKind` e
`raw_os_error` medidos na própria fixture; se rename e cleanup não forem
distinguíveis, o controle falha fechado em vez de atribuir crédito.

## Assinaturas e consumo da capacidade

O consumer fixa por type-check, sem depender de representação:

```rust
arm: fn(&[PathBuf]) -> ArmedWatch
ArmedWatch::publish:
    fn(ArmedWatch, &Path, &Path) -> io::Result<WatchSnapshot>
ArmedWatch::abandon:
    fn(ArmedWatch, &Path) -> WatchSnapshot
snapshot: fn(&[PathBuf]) -> WatchSnapshot
wait_for_change_since: fn(WatchSnapshot, Duration)
wait_for_change: fn(&[PathBuf], Duration)
```

As duas finalizações recebem `ArmedWatch` por valor e, portanto, consomem
`self`. `WatchSnapshot` e `ArmedWatch` são apenas transportados como valores
opacos; o teste não escolhe campos, layout, quantidade de entradas, algoritmo
de fingerprint ou forma named/tuple.

## Inventário público por rustdoc JSON pinado

O oráculo reaproveita a interface instrumental validada em P1295:
`RUSTC_BOOTSTRAP=1 cargo rustdoc -p typst-infra --lib --offline -- -Z
unstable-options --output-format json`, com cargo 1.92.0, rustc/rustdoc 1.92.0
e schema rustdoc JSON 56 estritamente pinados.

Ele localiza unicamente `typst_infra::watch::ArmedWatch`, exige tipo público
com storage privado não vazio e sem field público. O impl inherent deve expor
exatamente `publish` e `abandon`; traits não são usados como proxy de
representação ou proibidos sem obrigação produtiva correspondente. As
assinaturas consumidoras continuam provadas pelo type-check. Entre todas as funções livres
públicas que referenciam `ArmedWatch`, deve existir exatamente
`typst_infra::watch::arm`.

No módulo público `typst_infra::watch`, nenhuma função pública pode chamar-se
`commit_output` ou `discard_output`. Essa enumeração machine-readable mata
R1M3 sem busca textual de source e impede L4 de contornar a capacidade pelos
helpers crus. Exit não zero, versão/schema divergente, path/ID/kind ausente,
variante desconhecida ou JSON inválido falham fechado; não viram `Unknown` com
crédito.

Limitação explícita: rustdoc JSON e `RUSTC_BOOTSTRAP` são instáveis. A prova é
reproduzível somente para a toolchain e schema pinados. Mudança de qualquer pin
invalida este oráculo e exige nova medição, não adaptação silenciosa.

## Cobertura dos mutantes mínimos R1

- **R1M1 — paths + captura lazy em `abandon`:** rejeitado por A.
- **R1M2 — rename antes da captura em `publish`:** rejeitado por B.
- **R1M3 — L4 contorna capacidade por helpers crus:** rejeitado pelo inventário
  rustdoc que proíbe `commit_output` e `discard_output` públicos, em conjunto
  com as assinaturas consumidoras.
- **R1M4 — espera recaptura baseline:** rejeitado por A e B, cujas transições
  terminam antes da espera e não têm estímulo posterior.

P3 deve ainda executar controles e mutantes em ambas as ordens. Este contrato
não antecipa score, selo ou implementação.

## Proibições do harness

Não usar sleep de prontidão, carga artificial, tamanho/duração como sinal,
retry-until-pass, segunda alteração corretiva nem polling test-side como
evidência. `recv_timeout` é permitido exclusivamente para limitar waiter já
iniciado depois de uma transição concluída. O polling interno especificado de
`wait_for_change_since` não é sinal de prontidão do harness.

## RED e aceitação da fase P2-R1

Antes de implementação, executar uma vez o comando canônico:

```text
cargo test -p typst-infra --test p1297_watch_capability_contract -- --nocapture
```

O resultado deve ser RED determinístico de compilação porque a API R1 pública
não existe no baseline. Zero testes executados, ausência de timeout e
diagnósticos que nomeiem `ArmedWatch`/`arm` constituem causa válida. Falha de
dependência, rede, toolchain, fixture ou outro erro é ambiental e não autoriza
P3.

Após implementação futura, todos os positivos, fronteiras, type-check e
inventário pinado devem passar sem editar este Prompt ou consumer. A aceitação
cobre somente R1 e R1M1–R1M4; não certifica o watch geral, não absolve
P1294/P1295/P1296 e não substitui os contratos P1295/P1137.
