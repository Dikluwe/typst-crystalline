# Gate concreto — aspas semânticas e render contextual

Status: confirmado pelo humano em 2026-10-06; materialização concluída e validada.
Nome público na serialização e show da função nativa já corrigidos em fluxo
contínuo; não são aprovação nem materialização desta mudança de fase.

Medições em diagnosticos/smartquote-2026-10-06 registram o pin e o estado exato.
Aspas capturadas conservam glyph inglês após idioma de uso mudar; disabled
apaga a identidade; campos explícitos/omitidos confundem-se. HTML não aceita
SmartQuote programática, e perderia suporte a markup se mudássemos só eval.

Proposta revisável:

1. Eval conserva SmartQuote; glyph é escolhido durante render de Paged/HTML.
2. SmartQuoteElem.double muda bool → Option<bool>, e enabled:Option<bool> é
   adicionado. Omitido difere de true/false explícito. Signature Content::smartquote(bool)
   e assinatura Typst smartquote(...) ficam iguais.
3. Helper puro público SmartQuoteContext/resolve_smart_quote em lang/quotes
   centraliza seleção contextual; flags isolados de layout dão lugar a contexto
   por fluxo, restaurado em checkpoint, sem I/O/globais/cache persistente.
4. Fields/getter/JSON mostram apenas propriedades assentes; show usa identidade
   estática existente, sem NodeKind novo. HTML resolve quote semanticamente,
   sem entrar em paginação, mantendo escaping/raízes/raw guard.

Assinaturas integrais e contratos específicos estão nos L0s completos em
smartquote-semantica-2026-10-06/manifest.json, com hashes base/candidato e
consumer exclusivo 1:1. Não são L0s vigentes enquanto o gate estiver pendente.

ADR-0127 §2 exige parar para contrato público e eval ↔ layout/render. Confirmar
este pacote autoriza guardar L0s ratificados, RED→GREEN, implementar seus
consumers, ressellar apenas os owners afetados, build/lint e revalidação.
Não autoriza unsafe, nova flag/default, enum NodeKind, API de watch, math phase
ou outros gates pendentes. Root HTML lang metadata e Styled geral têm negativos
independentes registrados; não constituem critério de aspas universalmente fechado.

Invariantes compartilhados ficam no Núcleo Tekt proposto smartquote-semantics,
com TOML e SHA-256 efetivo registrados no manifesto. Os 24 prompts candidatos
possuem pin completo; o TOML só é instalado após confirmação junto de seus
consumidores, evitando núcleo órfão. Núcleo não materializa código, não tem
owner/Hash do Código/consumers e nunca é alvo de @prompt. Este grafo integra
o pacote sujeito à aprovação; nenhum arquivo novo foi criado no L0 vigente.

Execução e limites: `00_nucleo/diagnosticos/smartquote-execucao-2026-10-06.md`.
Receipt final: `00_nucleo/diagnosticos/smartquote-2026-10-06/final-checks.json`.
