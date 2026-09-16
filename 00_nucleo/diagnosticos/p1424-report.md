# P1424 — relatório independente

**Veredito: FAIL.** Executado sem atestação de isolamento.

O candidato passa as suítes seladas: 3/3 testes L1 e 4/4 testes L3 nas ordens `forward`, `forward_repeat` e `reverse`. `cargo fmt --all -- --check`, `cargo build`, `crystalline-lint .` e `git diff --check` encerraram com código 0. Os hashes do manifesto, contrato, oráculos, mutantes, selo e quatro ficheiros de testes continuam idênticos ao R3. Os 28 caminhos candidatos estão dentro da allowlist declarada; as alterações L0 são apenas resselo mecânico de hash, incluindo a primeira inserção do `Hash do Código` no owner novo.

O gate adversarial, porém, não fecha. O caso obrigatório `P13-unsupported-fails` exige que um corpo rejeitado pelo subset de layout/SVG produza o erro subjacente ou `Unknown` explícito. O teste selado O7 verifica apenas outra fronteira: `export_html` chamado diretamente com um `HtmlFrame` não resolvido. Esse teste rejeita M21, mas não M20.

O candidato materializa M20: o dispatcher paginado trata `Content::HtmlElem` como `no-op` em `01_core/src/compiler/layout/mod.rs:2425`; `layout_intrinsic` devolve uma `Page` sem canal de erro; e o resolvedor no pipeline serializa essa página como SVG. Assim, um corpo HTML-only dentro de `html.frame` pode terminar como ilha vazia/zero-size bem-sucedida, em vez de falhar ou ser classificado `Unknown`. A evidência é estática e conclusiva, sem promoção de `Unknown`.

Resultado de mutação: 20/21 ataques `Violated`, M20 `Preserved/SURVIVED`, score `0.9523809523809523` contra o mínimo `1.0`. Os quatro controles opacos permanecem `Unknown` com crédito zero, e os vetores foram iguais nas três ordens. O caso positivo obrigatório P13 é `Violated`; logo os testes verdes não autorizam o certificado.

Como regressão ampliada, os 17 testes unitários do exporter HTML passaram. A suíte de pipeline teve 60 passes e uma falha em `p1159_location_page_numbering_ve_func_no_relayout`; a mesma falha e a mesma asserção foram reproduzidas no baseline causal `79c7a6d60`, portanto é pré-existente e não foi atribuída ao P1424.

Para reabrir a verificação, é necessário invalidar o selo desde os oráculos/contrato afetados: acrescentar um fixture executável de corpo realmente não suportado, fazê-lo falhar de modo explícito no candidato e recalibrar M20 nas três ordens. Este relatório não corrige o candidato nem modifica entradas seladas.

Limite: o veredito cobre somente o fragmento P1424 e os 21 mutantes selados; não atesta paridade HTML/SVG geral.
