# P1305 — reabertura autorizada da construção do global

O dono respondeu `Autorizado` à pergunta que propôs ampliar o P1305 para
corrigir também a construção do global em `eval/mod.rs` e `eval/modules.rs`,
mantendo os demais limites. Esta autorização substitui exclusivamente a
proibição desses dois owners no plano original. O plano e os artefatos da
tentativa bloqueada ficam imutáveis para preservar seus hashes históricos.

A obrigação continua a ser a coorte de doze caminhos selecionada pelo P1304,
com elisão somente da representação, dados preservados e módulos ordinários
nomeados corretamente. Não há autorização de nova entidade/API, correção de
anonimato de plugin, encoders, export, warnings, spans ou diagnóstico de paths.

Após a contraprova registrada no certificado P1305 SHA-256
`008be227d322a764f56b368f42291b5fe4a057b3cc41f67ff4fc5fc2776442b4`,
será medida a hipótese de transportar o nome público `global` nos três pontos
existentes de construção, sem reconhecimento heurístico no formatter.
Medir aliases, imports bare/rename/wildcard e as três rotas antes de fixar o
contrato L0. Não presumir que a mudança de nome interno seja invisível.

Os owners adicionais são `00_nucleo/prompts/compiler/eval.md` e
`00_nucleo/prompts/compiler/eval/modules.md`, cada um com seu consumer 1:1.
O gate esperado é correção de paridade em fluxo contínuo ADR-0127, dentro da
ampliação explicitamente autorizada. Nova assinatura pública, campo, entidade,
default de produto ou mudança de fase exige nova decisão antes do código.

As autoridades e paths de saída estão declarados em
`p1305-r2-preflight-manifest.json`. O contrato/L0 será escrito antes da
implementação; oráculos e testes independentes serão congelados antes de ler
o patch. Regime: executado sem atestação de isolamento técnico.

Sem staging ou commit nesta execução: a autorização recebida trata da
ampliação e implementação, não de operações Git.
