Todos os perfis corresponderam às expectativas anteriores ao re-sync, com
UNKNOWN=0 e VIOLATED=0. A inversão de ordem foi idêntica nos seis perfis.
`matrix-expectations-integrity.json` confirma que casos, comandos, comparadores
e esperados são idênticos aos do checkpoint anterior.

Os dois DIFFERENCE permanecem registrados em todos os perfis:

- `P1138-X-002`: PNG com dimensões coincidentes, 11 de 2.005.644 pixels
  diferentes e variação máxima de um canal. O comparador de pixels continua
  classificando DIFFERENCE; a medição não demonstra defeito semântico.
- `P1138-X-003`: página, texto e geometria PDF coincidentes no observável do
  caso; nome interno de fonte `LibertinusSerif-Regular` versus
  `LibertinusSerif-Regular-Identity-H`. Não se exige copiar a mecânica Rust
  ou os bytes do PDF para obter paridade de linguagem.

Essas duas diferenças de formato não foram convertidas artificialmente em
MATCH. Os estados se referem somente a esta amostra e aos seus comparadores.
