# P1318 — GO independente pré-patch

Veredito: GO para implementar somente o recorte P1318 previamente revisado.
Revisão em `2026-09-08T15:33:21.987Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado;
diff/stat e fontes identificados pelos recibos abaixo. Nenhum candidato existia:
fonte atual `fece2c4d40c9acdcbae90b5a628f1cc650a5351826f095f0b22f1d674233ffd0`
e prefixo produtivo literalmente igual ao snapshot da medição.

L0 integral e normativo seguem respectivamente
`ee3e5a74aaad1a9add4ae16d4b9147634ad95cb9eee3fdfc624b720d93eb7f3e` e
`2b23a5adf4c7b62afbfb5ee9b8899bb756f36c2eb158c2ddb7e2795e49312ee6`.
A exclusão normativa é somente a linha canônica Hash do Código.

Manifesto de delta `p1318-test-delta.json`, SHA-256
`bfd34662ab8f82365127eacd264a88a5581064788de84e4cd5aff01f8920716f`:
comparação independente das funções com o snapshot confirmou seis testes
antigos atualizados, três novos e nenhuma função removida. Os corpos before
e after do manifesto coincidem com as fontes. As alterações antigas só
declaram o novo sufixo Bytes; causa, origem, detached, hints, traces e casos
continuam cobertos. O teste P1313 ganhou ainda asserção literal da mensagem
pura antiga. Path/Str e decoder público têm controle dedicado novo.

RED `p1318-unit-red.json`, SHA-256
`4a746ed84042f69917f25b58bef1e6eea417b957e1bb5cf74422f3b099cef910`:
`cargo test -p typst-core --release compiler::stdlib::loading::tests --lib`,
target `/tmp/p1318-target.eAgQwp`, exit 101. Entre os instantes registrados
`15:28:35.621514+00:00` e `15:30:49.726366+00:00`, fonte e L0 não mudaram.
Oito falhas são precisamente ausência do sufixo; 59 testes passam, incluindo
o novo controle puro/Path. Não é falha de compilação ou infraestrutura.

Freeze A/B `p1318-ab-freeze.json`, SHA-256
`aede13464afe720b4dffe501a9cab59f22f92a6101c12e9be7878c083abcb334`:
513 casos, incluindo 387 replays históricos; quatro perfis produzem 2052
expectativas, sendo 788 RED, 756 com igualdade vanilla integral e 32 efeitos
normativos de excesso. Todos os inputs congelados conferiram por SHA-256.
Reconstrução independente das expectativas a partir do baseline, alterando
somente o sufixo declarado, encontrou zero divergências.

Reparo writer `p1318-ab-baseline-runs-r1.json`, SHA-256
`06625b05fc79e023e26079b7871a399afddf3c15b48b2daac52e2528ff3f2be1`:
o original malformado é preservado e pinado. A reconstrução independente
substituiu exatamente oito LF literais em strings JSON por U+2028 e produziu
o mesmo objeto integral, exceto a metadata de revisão adicionada. Cada uma
das oito observações reparadas coincide integralmente com a execução focal
(exit/stdout/stderr/argv/cwd). Não houve alteração de expectativa nem novo
corpus completo. Isso corrige serialização, não o resultado do produto.

Regime A/B executado sem atestação de isolamento técnico. Limites do freeze
estão corretos: API pura, Args sintético Rust, offset impossível/fallback,
ausência de I/O e parser único também exigem testes locais e revisão de fonte.
Não há selo de refinamento nem alegação de paridade geral. Candidato,
GREEN, replay normal/repeat/reverse, build, testes workspace e lint ainda
dependem de execução e revisão final.
