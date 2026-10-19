---
# P615 — Reverter `DocumentID` para UUID aleatório

> **Passo:** 615
> **Data:** 2026-07-05
> **Foco:** P612/P613 implementaram `DocumentID` estável por hash de conteúdo, com a razão de "reconhecer revisões como o mesmo documento". Análise técnica posterior mostra que isto não cumpre esse objectivo (uma revisão real muda o texto, muda o hash, muda o ID — o oposto do pretendido), e cria um problema novo (documentos diferentes com conteúdo igual ficam com o mesmo ID). A norma ISO/Adobe para XMP, e o comportamento real do vanilla, é gerar um UUID aleatório no momento da compilação, fixo daí para a frente só dentro dessa mesma execução. Este passo reverte para essa abordagem.
> **Tipo:** Implementação directa. A razão já está confirmada, não é preciso sonda nova.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** A regra de decisão nova aplica-se ao contrário aqui — a decisão de P612/P613 foi tomada com boa fé, mas com análise incompleta; corrige-se agora com a mesma abertura.

---

## Contexto

`xmpMM:DocumentID` deve identificar a raiz conceptual do documento, mantendo-se igual ao longo de revisões reais (o mesmo ficheiro fonte, editado e recompilado continua "o mesmo documento"). `xmpMM:InstanceID` identifica cada gravação/compilação específica, mudando sempre.

O cristalino, tal como o vanilla, não tem estado entre compilações — cada execução do compilador não sabe nada sobre execuções anteriores do mesmo ficheiro fonte. Sem esse estado, não há forma correcta de detectar "isto é uma revisão do que compilei ontem" apenas a partir do conteúdo, porque:

1. Uma revisão real muda o conteúdo, e por isso mudaria o hash — quebrando a continuidade que se queria manter.
2. Conteúdo igual por coincidência (dois documentos de propósitos diferentes, mas com o mesmo texto) receberia o mesmo ID, misturando identidades que deviam ser distintas.

A prática correcta, seguida pelo vanilla: gerar um UUID aleatório (RFC 4122, versão 4) a cada compilação, para os dois campos — `DocumentID` e `InstanceID` — já que não há memória de execuções anteriores para justificar manter o `DocumentID` estável.

---

## Implementação

Reverter `xmp_instance_and_document_id` (`03_infra/src/export/builder.rs`, introduzida em P612) para gerar dois UUIDs aleatórios independentes, um para `InstanceID`, outro para `DocumentID`, sem hash de conteúdo nenhum.

```rust
// Esboço, a confirmar contra a estrutura real:
fn xmp_instance_and_document_id() -> (String, String) {
    if std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH").is_ok() {
        return (FIXED_INSTANCE_ID.to_string(), FIXED_DOCUMENT_ID.to_string());
    }
    let instance = generate_uuid_v4();
    let document = generate_uuid_v4();
    (instance, document)
}
```

Confirmar se já existe uma dependência de geração de UUID no projecto (`uuid` crate, ou equivalente), ou se precisa de ser adicionada.

### Formato do UUID no XMP

Confirmar se o vanilla usa o formato `uuid:xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` (com o prefixo `uuid:`) ou só base64, como P612 tinha implementado — a sonda de P611/P612 já mostrou exemplos do vanilla (`q7a5Evax/4zCi9aVbPFo9w==`, formato base64, não UUID textual). Confirmar qual formato o vanilla usa de facto antes de mudar, para não trocar um formato errado por outro.

### Critério de fecho da implementação

- [ ] `DocumentID` e `InstanceID` gerados aleatoriamente, sem hash de conteúdo.
- [ ] Formato confirmado contra o vanilla (base64 de 16 bytes aleatórios, ou UUID textual — o que P611 já mediu do vanilla).
- [ ] Documentos diferentes produzem IDs diferentes (já confirmado antes, deve continuar).
- [ ] O mesmo documento, compilado duas vezes, produz `DocumentID` diferente nas duas vezes (o comportamento correcto, ao contrário do que P612/P613 tinham decidido).
- [ ] Valores fixos continuam a funcionar sob `CRYSTALLINE_PDF_FIXED_EPOCH`, para os snapshots de teste.

---

## Validação

```bash
cat > /tmp/p615-mesmo.typ <<'EOF'
Documento igual, compilado duas vezes.
EOF
./target/release/typst /tmp/p615-mesmo.typ /tmp/p615-1.pdf
sleep 1
./target/release/typst /tmp/p615-mesmo.typ /tmp/p615-2.pdf
```

Confirmar que os dois `DocumentID` são diferentes agora — o oposto do que P613 tinha confirmado como comportamento do cristalino, e igual ao que P613 já tinha confirmado como comportamento do vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `DocumentID`/`InstanceID` gerados aleatoriamente, sem hash de conteúdo.
- [ ] Mesmo documento, duas compilações, dois `DocumentID` diferentes — paridade real com o vanilla, não só semântica.
- [ ] Formato do valor confirmado contra o vanilla.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p615.md`, com hash do commit.
- [ ] Prompt L0 (`00_nucleo/prompts/infra/export/builder.md`) corrigido — a nota de "divergência intencional" de P613 é removida, substituída pela explicação de porque a divergência estava errada, não pela decisão que a mantinha.
