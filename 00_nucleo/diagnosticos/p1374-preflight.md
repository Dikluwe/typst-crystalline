# P1374 — preflight de materialização

## Regime e autoridade

Protocolo completo executável somente **sem atestação de isolamento**: esta
autoridade leu intenção, fontes, L0 e controla contrato, futuro candidato e
gates. Hashes demonstram identidade e ordem, não independência forte.

## Baseline medido

Commit `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado. Vanila `eb60986b…1468f`; cristalino `0b318c3f…4e15`;
`pdftotext` `0fb98ea1…565e7`. Execuções sequenciais: 2 GiB por compilador,
512 MiB para `pdftotext`, arquivo 16 MiB e timeout 45 s. Nenhum OOM, timeout,
SIGKILL, Cargo ou suíte ampla.

## Constatação

Width percentual é perda de `Rel`; height fracionária exige carrier distinto
e distribuição entre siblings. Reutilizar `Option<Rel<Length>>` exigiria uma
sentinela proibida. O delta público é inevitável para representar a linguagem.

## Ordem congelada

1. confirmação humana ADR-0127;
2. resselo de manifesto/contrato/oráculos;
3. testes RED focais;
4. implementação do carrier, transporte, resolução e primeira fatia do flow;
5. testes GREEN, matriz bilateral e ataques;
6. hashes, lint e certificado limitado.

Nenhuma etapa 3–6 está autorizada antes da confirmação.
