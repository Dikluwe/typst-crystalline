# P1327-R2 — gate independente antes de C2

Regime A/B sem atestação técnica de isolamento. Gate **PASS** para C2 no
recorte `run_eval` autorizado; ainda não é closure ou GREEN final.

## Identidades e proveniência

- Manifesto R2 verificado:
  `9ebf13c2cf9a1c1daf186671194c7c4e15db16d8d4e47a12ad5ff4fb597f8ad0`.
- Oracle adicional verificado:
  `19d38cbf430e65a3ca67d2414ee8623f9fc7111e5e4077d4499e1d67e0d88984`.
- Runner adicional:
  `97529a351d83c19416538d91c5173ec5cead8efa39aeee82477a24fdffa019ed`.
- Medição adicional:
  `46abf67a60c7ebb04703cf70d6392b5927ac3e21bbab47753850aa6b1e34edf2`.

Manifesto liga baseline R2 `84056881…`, original, RED CLI C1 e seu
veredito Violated; conjunto de três pares de owners explícito. As entradas
normativas modules/tests mantêm seus hashes originais; wiring congela
`7c19de0b005fadc58987e544d55c09a3cdcc806ef5c7a65ae89d5643cd7aa0b5`.
Todos recalculados independentemente sem diferenças. Main.rs ainda coincide
com o baseline R2 excluído header: não há C2 nesta revisão.

Proveniência working tree do HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, com estado completo em baseline
R2 e manifesto. Runs adicionais entre 2026-09-09T11:08:02.733075 e
11:08:05.750725 UTC; freeze 11:09:08.849079 UTC. A medição verifica C1
SHA `5f00502b…` e vanilla ratificado SHA `7b4f40c5…`; o manifesto conserva
também cópia imutável do C1 em `/tmp/p1327-c1.ZvH24a/typst`.

## Reclassificação independente

Li o runner completo e recalculei todos os 32 pares novos, 64 execuções.
As 32 chaves `(perfil,id)` são únicas, completas e têm expected correto:

- Três correções por perfil: warning de deprecação seguido de erro,
  warnings mistos de deprecação/import e dois warnings de deprecação em
  spans diferentes. Todos preservam exit/stdout e blocos stderr; somente
  deslocar o bloco error antes dos warnings produz o vanilla exato.
- Três controles de paridade por perfil: warning sem erro, erro sem
  warning e sucesso sem warning. Transcripts integrais coincidem.
- Dois controles de serialização raw por perfil: preservam C1 integral
  porque eval retorna Ok e a falha ocorre depois. Diferenças preexistentes
  `int`/`integer` e emissão do warning continuam dívida explícita, conforme
  a obrigação de preservar o branch Ok; expected não é derivado de C2.

Runner compara exit/stdout/stderr integralmente, sem ordenação artificial,
normalização ou descarte de warning. A dupla de raw não é usada para alegar
paridade. O corpus original permanece imutável e suas 24 falhas CLI C1
continuam RED causal válido, sem retrospectivamente promover R0.

A cobertura nova discrimina solução limitada à mensagem de import, perda
de warning preexistente, perda/reordenação de múltiplos warnings e efeito
indevido no branch Ok. Junto ao RED original e controles já auditados,
é suficiente para iniciar C2 no owner wiring. Após C2 exigir GREEN de ambos
os corpora, preservação dos testes congelados, diffs de produto delimitados,
gates arquiteturais e linhagem antes de closure.
