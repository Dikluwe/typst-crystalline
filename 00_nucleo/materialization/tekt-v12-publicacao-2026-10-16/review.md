# V12 produtivo — transporte de publicação

Proposta para retirar o enum privado Published de L4 e defini-lo como `typst_infra::watch::Published`, em L3, com exatamente `Html(String)` e `Bundle(Vec<BundleFile>)`. São os mesmos payloads e o mesmo dispatcher; o import da raiz L4 permanece privado. Não altera flags, defaults, resultados, fase, servidor, API existente nem publicação de release. Não há ganho de RAM alegado.

O L0 atual wiring.md autoriza explicitamente o carrier em L4. Os quatro L0 propostos neste pacote atualizam essa decisão: wiring.md, wiring/compile.md, wiring/watch.md e shell/watch.md (owner de 03_infra/src/watch.rs). Os arquivos completos em l0/ usam seus caminhos canônicos. Hash do Código será ressellado somente após materialização ratificada; os SHA-256 das propostas estão no manifesto.

A aprovação cobre apenas esse novo contrato público e sua integração mínima. O enum fica acessível a clientes Rust externos; retirá-lo depois pode quebrar esses clientes. Por isso a ADR-0127 §2.1/§2.4 exige confirmação antes do código. Não inclui publicação de release. Os 18 V12 de suporte de testes seguem uma frente interna independente, com L0 próprio e sem API pública de produto.

T1: controles reais de watch HTML/bundle, HTTP/reload e preservação após erro. T2: controles de compile HTML/bundle e assets/status/diagnósticos, sem alterar esperados. T3: V12 RED→GREEN, diff restrito e validação completa. Os L0 especificam os testes concretos existentes; não acrescentar testes que apenas repitam a forma do enum. Todos os jobs pesados sob 12 GiB, swap zero e um job.

Medição e estado inicial: 00_nucleo/diagnosticos/tekt-v12-2026-10-16/before.json.gz e scope.json. Nenhum código do contrato público foi escrito antes da aprovação.
