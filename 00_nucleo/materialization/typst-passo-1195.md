# P1195 — separar porta e adapter de package download

**Estado:** EXECUTADO — GREEN EM 2026-08-25
**Baseline condicionado:** P1194 GREEN; V15=12.

Criar `contracts/package_downloader.md` para o trait L1 e manter
`infra/package_downloader.md` para o adapter L3. Extrair para
`_nuclei/packages/downloader-contract.toml` somente as obrigações de fronteira
que ambos devem honrar; I/O/rede é exclusivo L3 e nunca entra no owner L1.

Gate: V15 12→11, V26=0, topologia V3/V4/V14 inalterada, V5 focal limpo,
testes de porta/adapter, build e dry-run bloqueado por 11 V15. Diagnóstico:
`typst-p1195-saneamento-package-downloader.md`.
