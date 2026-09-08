# Passo 1311 — campo ausente em função nativa sem namespace

## Medição e escolha

P1310 fechou o cast de fonte dos cinco decoders. A próxima coorte elegível
na ordem registrada em `00_nucleo/diagnosticos/p1309-selection.json` é
`plain-function-missing-field`, rank `[3,1,-3,1,id]`: `csv.encode`,
`read.encode`, `xml.encode`. Não se agrupa o restante dos erros de field access.

Revalidação fresca: `00_nucleo/diagnosticos/p1311-measurement.json`, SHA-256
`f444074ac6c1e8be4509b08eed480ceb1e0188f7e8afec892bdaa31938a3d6c3`,
em `2026-09-08T00:48:36.611014Z`, HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39` com P1310 não commitado.
`csv.encode` e `csv.encode(1)` no vanilla ratificado `a51e02804` dão
``function `csv` does not contain field `encode` `` com span em `encode`;
o baseline P1310 dá erro genérico e span em `csv.encode`.
Alias/With preservam o nome `csv`; `calc.abs.nope` publica `abs`.

Fonte: vanilla `foundations/func.rs:280-308` e `typst-eval/src/code.rs:347-367`;
cristalino `compiler/eval/bindings/field_access.rs:103-112,254-268`.
Mensagem e âncora são linguagem diagnóstica (ADR-0108), não igualdade interna.
Inferência: discriminar a categoria já representada em Func e usar o field span
basta sem mudar entidade/dispatch. Uma rota que exija outro owner a refuta.

## Escopo autorizado

Único owner: `01_core/src/compiler/eval/bindings/field_access.rs`.
L0: `00_nucleo/prompts/compiler/eval/bindings/field_access.md`.
Nativas Native/NativeWithEngine sem namespace, inclusive With aninhado:
nome público da função, field solicitado e span somente do field ausente.
Sem lista especial de csv/read/xml/encode nem classificação por nome lexical.
O nome nativo qualificado é projetado para seu último segmento público.

Não criar encoder/field; não alterar namespace, função ou forma de chamada.
Namespaces presentes, closures, plugins e elementos de usuário ficam com
seus comportamentos anteriores; Dict, Module, Content, Type, PDF/features,
float/is-nan e warnings são controles de preservação. Não corrigir incidentalmente
as demais mensagens ou âncoras. Preservar integralmente P1310, inclusive a
dívida Symbol e a ordem de validação named documentadas.

ADR-0127: correção interna de paridade em fluxo contínuo após L0-first.
Sem novo campo, trait, assinatura pública, default, compatibilidade ou fase.

## Execução e aceitação

Regime A/B proporcional da skill `tekt-materializacao-segregada`: root escreve
L0/código/testes unitários; testador independente mede e congela expectativas
sem ler o patch; revisor distinto audita escopo/casos **antes do candidato**
e emite veredito final sem editar solução ou testes. Filesystem compartilhado:
**executado sem atestação de isolamento técnico**. Não alegar selo de
refinamento completo ou score de mutação de produto.

1. Baseline explícito inclui P1310 não commitado, hashes dos arquivos e evidências
   anteriores. Target RAM próprio, sem sobrescrever executável precedente.
2. L0 atualizado antes de Rust; validar ownership/Núcleos e resselo.
3. Medir os quatro perfis, congelar a suíte e revisar suas políticas antes do
   patch. Testes locais `#[cfg(test)]` no owner; registrar RED real.
4. Corrigir a causa; GREEN local; candidato contra expectativas bilaterais
   congeladas nas ordens normal, repetida e inversa. Preservar raw stdout/stderr,
   mensagem, hints, ranges e traces; não normalizar falhas para igualdade.
5. Revalidar a suíte P1310 e controles P1308, distinguindo deltas intencionais
   deste passo dos fechamentos anteriores e das dívidas preservadas.
6. Rodar cargo fmt/check, build workspace release, testes workspace release,
   crystalline-lint e git diff/check. Reportar warnings/info e ignorados.
7. Verificador recalcula contagens e hashes, comprova preservação de P1310 e
   mudanças limitadas ao owner/L0. Relatório substantivo em
   `00_nucleo/diagnosticos/p1311-final-report.md`.

Timeout, crash, fixture/binário trocado ou observável obrigatório ausente =
Unknown bloqueante. Até duas revisões focais por causa; duas sem ganho exigem
reexaminar escopo/observabilidade, não repetir o corpus inteiro ou afrouxar testes.
Não reutilizar percentuais globais P1309 como medição nova.

Escritas: este passo, owner/L0 citados, evidências `p1311-*` em diagnósticos e
temporários dedicados. Sem stage, commit, push, limpeza histórica ou passo 1312.
