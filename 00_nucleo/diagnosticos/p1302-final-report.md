# P1302 — relatório final de sanitização e pré-commit

## Resultado

O primeiro staging literal dos 83 paths foi interrompido antes do commit porque
`git diff --cached --check` encontrou 33 ocorrências de whitespace final em sete
documentos que eram não rastreados antes do staging. O veredito então emitido foi
`P1302_BLOCKED_STAGING_MISMATCH`; o índice ficou com os 83 paths staged e não houve
commit.

Em 2026-09-04, o dono autorizou explicitamente que o próprio P1302 corrigisse essa
falha mecânica e a registrasse neste relatório. A correção removeu somente os 33
espaços horizontais finais, sem alterar código, L0, testes, contratos ou semântica.

Paths sanitizados:

- `p1300-l0-gate-receipt.md`: 12 ocorrências;
- `p1300-p5-public-api-note.md`: 2;
- `p1300-red-tests-receipt.md`: 2;
- `p1301-implementation-receipt.md`: 2;
- `typst-passo-1299.md`: 4;
- `typst-passo-1300.md`: 7;
- `typst-passo-1302.md`: 4.

O patch cached bloqueado tinha SHA-256
`96d83bef15bd18aa3fbc3f60593f9f662fa430e888598be7941383150b1e66a3`.
Os recibos P1 e P2 não foram reescritos: permanecem evidência do estado anterior à
sanitização. O manifesto de staging e o certificado P1302 foram revisados para
registrar a exceção autorizada e os novos hashes dos sete inputs.

## Revalidação proporcional

Como o delta é exclusivamente whitespace em Markdown, a cadeia semântica não foi
reexecutada. Os oito hashes de produto/L0 permaneceram exatamente os pinados pelo
P1302. Depois da sanitização passaram:

- `cargo fmt --all -- --check`;
- `crystalline-lint --fix-hashes --dry-run .` → `Nothing to fix`;
- `git diff --check`.

O staging deve agora ser atualizado somente pelos 83 paths literais do manifesto.
Depois disso, `git diff --cached --check`, igualdade exata da allowlist, ausência de
unstaged e a mensagem de commit continuam gates obrigatórios.

## Pins revisados

- `p1302-staging-manifest.json`:
  `00d9dab73cb55be6a4797b2f42b4608199e9db6c545c6621d840e9bfb089236a`;
  28.909 bytes.
- `p1302-certificate.json`:
  `05eded68bfe3a790b2cc1fdcef353c33bfa3175fc6593d2a853e8c815df17ac8`;
  6.229 bytes.

Política de identidade: DAG detached/non-recursive `manifesto → certificado →
relatório`. O hash deste relatório é medido depois da criação e não é inserido no
próprio ficheiro.

Regime de fechamento mecânico, executado sem atestação de isolamento técnico. A
alegação continua limitada à cadeia P1299–P1301r2; não se alega paridade funcional
geral e o bloqueio histórico P1300 não é reescrito.

## Veredito

`P1302_READY_FOR_EXACT_STAGING_AND_COMMIT_AFTER_OWNER_AUTHORIZED_SANITIZATION`
