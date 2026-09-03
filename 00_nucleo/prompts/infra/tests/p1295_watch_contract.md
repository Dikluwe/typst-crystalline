# Prompt L0 — contrato externo P1295 do snapshot de watch
Hash do Código: 771a5a67

**Camada:** L3 (integration test externo)
**Ficheiro alvo exclusivo:** `03_infra/tests/p1295_watch_contract.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129
**Origem causal:** W1 confirmado em `shell/watch.md`; gate humano `Continue`

## Propriedade e independência

Este Prompt possui exclusivamente o integration test externo acima. Ele não
legitima `03_infra/src/watch.rs`, não prescreve a representação do snapshot e
não pode ser adaptado a uma implementação candidata. O autor recebeu somente
W1/W2/W3 confirmados, manifesto, recibo do gate, baseline anterior a W1 e os
manifests necessários para executar o teste; nenhuma implementação candidata
existia ou foi lida.

O workspace é compartilhado e não fornece isolamento técnico de leitura. A
independência alegada é somente a segregação causal por entradas, capacidades,
ordem e allowlist de escrita registrada no manifesto P1295.

## Medição anterior à decisão

No baseline congelado em HEAD
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, `03_infra/src/watch.rs` possui
`Fingerprint` privado e distingue metadata, tamanho e hash de conteúdo, mas
expõe apenas `wait_for_change(paths, interval)`. Essa função captura o baseline
dentro da espera; não existem `WatchSnapshot`, `snapshot` ou
`wait_for_change_since` públicos.

W1 confirmado exige que a captura possa preceder a publicação, que a espera
consuma exatamente o snapshot fornecido sem recaptura, que criação, alteração
de mesmo tamanho e remoção sejam observadas, e que a API legada permaneça.

Classificação ADR-0107/0108: nomes de campos, algoritmo de hash, quantidade de
polls e representação da coleção são mecânica. Retornar para as três mudanças
e preservar a fronteira pública opaca são observáveis do contrato. Inferência:
aplicar o estímulo depois de `snapshot` e antes de iniciar
`wait_for_change_since` distingue a API correta de uma espera que recapture.
Refutador: se uma espera correta puder ignorar uma dessas mudanças já ocorrida,
W1 precisa ser reaberto; o teste não será afrouxado para acomodar o candidato.

## Oráculos positivos

Cada caso usa path temporário exclusivo e limpeza RAII:

1. escreve conteúdo inicial, captura o snapshot, troca por conteúdo diferente
   com o mesmo comprimento e só então inicia `wait_for_change_since`;
2. captura um path ausente, cria-o e só então inicia a espera;
3. captura um path existente, remove-o e só então inicia a espera;
4. transporta `WatchSnapshot` como valor opaco e fixa por type-check as
   assinaturas públicas de `snapshot`, `wait_for_change_since` e
   `wait_for_change`, preservando a API legada;
5. em Linux, entrega a `wait_for_change` o pseudo-ficheiro público
   `/proc/sys/kernel/random/uuid`, cujo conteúdo muda em cada leitura, e exige
   retorno. Assim a compatibilidade legada é exercida sem sleep de prontidão,
   corrida entre captura e estímulo ou escrita corretiva repetida.

Nos casos de espera, a função roda em thread descartável e deve sinalizar
retorno dentro de timeout explícito de dois segundos, com polling de cinco
milissegundos. O timeout é limite de falha, não sincronização: nos três casos
de snapshot toda mudança já foi concluída antes da thread começar, e no caso
legado o próprio pseudo-ficheiro fornece identidade diferente a cada leitura.
Não há sleep de prontidão, aumento de timeout, retry-until-pass ou segunda
alteração corretiva.

## Revisão arquitetural da opacidade pública

P3 demonstrou que a sonda original era inválida: em Rust 1.92,
`let WatchSnapshot { .. } = token` compila para structs públicas com estado
privado, inclusive named-private com `#[non_exhaustive]` e tuple-private. O
controle válido preservou cinco de seis casos e foi rejeitado somente por esse
falso negativo. `ORACLE_OPACITY_PATTERN` consumiu duas revisões sem ganho e
fica encerrado; não pode receber outra variação de pattern ou layout.

Novo `reason_code`: `ORACLE_OPACITY_STABLE_ENUMERATION_UNAVAILABLE`.

Hipótese de redesenho: provar simultaneamente “tipo opaco fora de L3” e
“nenhuma mutação pública das fingerprints” de forma neutra à representação
exige um inventário exaustivo, machine-readable e estável da superfície
pública do tipo. Sondas de compilação só podem perguntar por nomes ou formas
previamente escolhidos; elas não quantificam campos e métodos desconhecidos.
Rust estável não oferece reflexão de campos/métodos, e o `rustdoc` disponível
expõe apenas HTML sem contrato de formato. Rustdoc JSON requer nightly e
`cargo-public-api` não está disponível, logo nenhum deles é base reproduzível
autorizada para este contrato.

Refutadores que permitem reabrir este reason code: (1) uma interface estável e
machine-readable da toolchain pinada que enumere visibilidade, campos e métodos
do tipo; ou (2) nova decisão humana/L0 que torne nominal e finita a superfície
permitida, de modo que sondas externas possam ser completas sem escolher
representação de implementação. Até um desses fatos existir, a propriedade é
inobservável de forma robusta pela API estável disponível e bloqueia o selo.

O teste preserva somente o observável positivo neutro: `WatchSnapshot` pode
ser obtido por `snapshot`, transportado como valor opaco e consumido por
`wait_for_change_since`. Isso não recebe crédito como prova de privacidade nem
de ausência de mutadores. Não se aceita falha ambiental, erro de dependência,
HTML de rustdoc, nightly, busca textual do source, nome inventado de campo,
tuple index, trait acidental ou ausência de um único método como substituto de
inventário exaustivo. `Unknown` nunca vira sucesso.

### Novo refutador — rustdoc JSON pinado

Uma medição posterior refutou apenas a indisponibilidade instrumental:
`RUSTC_BOOTSTRAP=1 cargo rustdoc -p typst-infra --lib -- -Z
unstable-options --output-format json` funciona com a toolchain pinada. Isso
não torna rustdoc JSON uma interface estável nem absolve
`ORACLE_OPACITY_PATTERN`; cria a classe independente
`ORACLE_OPACITY_PINNED_RUSTDOC_JSON`.

O microcontrole P2, gerado fora do repositório em `/tmp`, foi processado por
rustdoc 1.92.0 e schema `format_version = 56`. Seu source SHA-256
`32def364bd3c2a8f5fb004a959bdc58b3da0ec6a643afcdda6d7aeafca85ca51`
produziu JSON SHA-256
`8c85d2b7675af011acde6d2435e23b463aa59ef6eade2141d5126c175cfb1c28`.
A medição distinguiu, sem ler nomes de campo: named-private por `fields=[]` e
`has_stripped_fields=true`; tuple-private por slots `null`; campo público por
ID não nulo; três métodos inherent públicos por seus IDs; e impl direto
`AsMut` separadamente de impls automáticos e blanket.

Hipótese: sob rustc/rustdoc 1.92.0, cargo 1.92.0 e schema 56 pinados, esse JSON
é inventário machine-readable exaustivo suficiente para o fragmento. O
oráculo localiza unicamente `typst_infra::watch::WatchSnapshot`, exige
visibilidade pública e estado privado não vazio tanto para forma named quanto
tuple, e rejeita todo field público. Percorre todos os IDs de `impls`: impl
inherent deve ter zero item público; trait impl direto deve ser zero; somente
auto traits e blanket impls podem permanecer. Finalmente percorre todas as
funções públicas cujo signature referencia o ID do snapshot e exige
exatamente `snapshot` e `wait_for_change_since`; `wait_for_change` permanece
fixada separadamente pelo type-check de compatibilidade.

Essas regras mantêm o transporte somente pelas funções livres confirmadas e
rejeitam getters, mutadores, associated items, traits diretos e funções livres
adicionais capazes de receber ou devolver o token. Não escolhem nome de campo,
forma named/tuple, quantidade de campos, tipo de fingerprint ou layout.

O comando usa `--offline` e `CARGO_NET_OFFLINE=true`, remove JSON anterior
antes da execução e fixa strings completas de cargo, rustc e rustdoc, além do
schema 56. Exit não zero, versão divergente, ficheiro ausente, JSON inválido,
ID/path/visibilidade/kind/impl/signature ausente ou variante desconhecida são
falha, nunca sucesso ou crédito `Unknown`.

Limitação explícita: `RUSTC_BOOTSTRAP=1` e `-Z unstable-options` exercem uma
interface instável, toolchain-specific. O oráculo é reproduzível somente para
B1/toolchain/schema registrados e precisa ser revalidado se qualquer pin
mudar. Refutador: se um controle representation-neutral correto for omitido ou
rejeitado, ou uma superfície pública nociva não aparecer/rejeitar, o parser é
insuficiente e a cadeia volta a bloqueio sem terceira adaptação local.

## RED e aceitação

Antes de implementação, `cargo test -p typst-infra --test
p1295_watch_contract` deve falhar deterministicamente na compilação porque W1
ainda não existe. Esse RED é válido e deve registrar comando, exit code,
diagnóstico, timestamp, HEAD, dirty stat e hashes das entradas/saídas.

Após implementação, os cinco observáveis funcionais e o inventário pinado
devem passar sem editar este Prompt ou o teste. O novo oráculo volta à
discriminação P3; não autoriza implementação nem selo por si mesmo. A aceitação
cobre somente o fragmento W1 registrado; não declara equivalência geral do
watch nem substitui o observador W3 em `04_wiring/tests/cli.rs`.
