# Prompt L0 — export HTML: pretty
Hash do Código: 8ee147b8

**Camada:** L3
**Ficheiro alvo:** `03_infra/src/export/html/pretty.rs`
**Atualizado em:** 2026-09-26

## Medição anterior à decisão

No snapshot anterior `03_infra/src/export/html.rs:250-415 tokeniza markup e formata fronteiras estruturais com aspas protegidas.`
A localização é mecânica L3 (ADR-0109); preservar a morfologia HTML e
mensagens de erro observáveis (ADR-0107), sem inferir intenção do produto.

## Responsabilidade e invariantes vigentes

Possui HtmlTokenKind/HtmlToken, pretty_html, push_structural_newline, is_protected_html_context, html_tokens, quoted_tag_end e classify_html_tag. Preservar varredura e classificação atuais, pilha, dois espaços por depth, newline final e proteção de phrasing/p/headings/title/pre/textarea/script/style/td/th. > entre aspas não encerra tag. O pass conserva bytes de cada token e não altera a string compacta antes da formatação. Não corrigir tokenizer de raw SVG/script neste refactor; mudanças de comportamento exigem teste e spec separada.

Módulo privado descendente de export::html; helpers usados entre irmãos
empregam pub(super), sem pub(crate), API nova ou mudança de fase. Mantém as
assinaturas, algoritmos e diagnósticos do snapshot medido, incluindo
adaptação local existente a Content::Styled(styled). A extração é interna
e segue fluxo contínuo ADR-0127; cada consumer possui owner 1:1 ADR-0129.

## Aceitação

Testes funcionais existentes e novas caracterizações passam antes/depois;
comparação de assinatura/corpo detecta deriva na movimentação. Não afirmar
ganho de tempo nem paridade nova a partir desta reorganização.

## Proveniência histórica das regras transferidas

As medições abaixo pertencem aos pins e passos históricos citados; não
constituem nova medição contra upstream atual. Estados de proposta e scope-out
antigos são superados pelos invariantes vigentes acima e cortes posteriores.

## P1360 — whitespace estrutural opcional

Adicionar `export_html_with_serialization_and_pretty(content, mode, pretty)`;
as funções anteriores delegam com `pretty=false`. Quando true, o encoder
insere newline final e indentação de dois espaços somente em fronteiras
estruturais de documento/bloco/head metadata. Conteúdo phrasing, texto,
`title`, atributos e espaços semanticamente protegidos não recebem whitespace
novo no seu interior.

Compacto e pretty devem produzir o mesmo DOM ordenado e os mesmos escapes no
modo selecionado. O formatador identifica o fim de tags respeitando aspas em
atributos; `>` dentro de atributo não encerra token. Void, raw, erros e
subset suportado permanecem iguais.
