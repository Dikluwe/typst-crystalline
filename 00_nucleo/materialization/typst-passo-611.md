---
# P611 — Stream de metadados XMP

> **Passo:** 611
> **Data:** 2026-07-05
> **Foco:** Registado desde P536 como "só /Info é emitido; XMP cobre o caso comum", sem custo real pesado contra benefício por escrito — na revisão feita depois de P594, ficou classificado como falta de implementação disfarçada de decisão. Este passo confirma o que o vanilla escreve exactamente no stream XMP, e implementa o equivalente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M–L. Requer gerar XML válido segundo a especificação XMP da Adobe, o maior dos itens desta lista.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P536 (implementação de `/Info`, XMP deixado de fora), P600/P601 (correcções mais recentes na mesma área de metadados).

---

## Sonda

### Confirmar o conteúdo exacto do stream XMP do vanilla

```bash
cat > /tmp/p611-teste.typ <<'EOF'
#set document(title: "Documento de Teste", author: "Autor Teste", keywords: ("chave1", "chave2"))
Texto.
EOF
lab/typst-original/target/release/typst compile /tmp/p611-teste.typ /tmp/p611-vanilla.pdf
mutool extract /tmp/p611-vanilla.pdf
cat *.xml 2>/dev/null || python3 -c "
import re
data = open('/tmp/p611-vanilla.pdf', 'rb').read()
start = data.find(b'<?xpacket')
end = data.find(b'<?xpacket end') 
if start != -1:
    print(data[start:end+50].decode('utf-8', errors='replace'))
else:
    print('Nenhum xpacket encontrado')
"
```

### Confirmar se o XMP é obrigatório sempre, ou só quando há metadados

```bash
cat > /tmp/p611-sem-metadados.typ <<'EOF'
Texto sem metadados.
EOF
lab/typst-original/target/release/typst compile /tmp/p611-sem-metadados.typ /tmp/p611-vanilla-sem.pdf
python3 -c "
data = open('/tmp/p611-vanilla-sem.pdf', 'rb').read()
print('Tem xpacket:', b'<?xpacket' in data)
"
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p611-teste.typ /tmp/p611-cristalino.pdf
python3 -c "
data = open('/tmp/p611-cristalino.pdf', 'rb').read()
print('Tem xpacket:', b'<?xpacket' in data)
"
```

### Critério de fecho da sonda

- [ ] Conteúdo exacto do XML XMP do vanilla confirmado, campo a campo (namespaces usados, estrutura RDF).
- [ ] Confirmado se o XMP é sempre emitido, ou só condicional a metadados (seguindo o mesmo padrão já corrigido para `/Info` em P601).
- [ ] Confirmado que o cristalino não emite XMP hoje.

---

## Implementação

### Construir o XML XMP

Estrutura mínima esperada (a confirmar contra a sonda, não assumir):

```xml
<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
      xmlns:dc="http://purl.org/dc/elements/1.1/"
      xmlns:xmp="http://ns.adobe.com/xap/1.0/"
      xmlns:pdf="http://ns.adobe.com/pdf/1.3/">
      <dc:title><rdf:Alt><rdf:li xml:lang="x-default">...</rdf:li></rdf:Alt></dc:title>
      <dc:creator><rdf:Seq><rdf:li>...</rdf:li></rdf:Seq></dc:creator>
      <xmp:CreatorTool>...</xmp:CreatorTool>
      <xmp:CreateDate>...</xmp:CreateDate>
      <xmp:ModifyDate>...</xmp:ModifyDate>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>
<?xpacket end="w"?>
```

A estrutura exacta (nomes de namespace, ordem de campos) tem de vir da sonda, não deste esboço.

### Emitir como stream no PDF

- Criar um objecto PDF `/Type /Metadata /Subtype /XML`, com o XML como conteúdo do stream.
- Referenciar esse objecto a partir de `/Metadata` no dicionário do catálogo do documento.

### Critério de fecho da implementação

- [ ] XML XMP gerado com a estrutura confirmada pela sonda.
- [ ] Stream de metadados emitido e referenciado correctamente no catálogo.
- [ ] Segue a mesma regra já estabelecida em P601 (emitir sempre, ou só condicional, conforme confirmado na sonda).
- [ ] Caracteres não-ASCII no XML tratados correctamente (reaproveitar a disciplina já estabelecida em P538b para `/Info`, mas em XML precisa de escape de entidades, não de codificação hexadecimal).

---

## Validação

```bash
./target/release/typst /tmp/p611-teste.typ /tmp/p611-depois.pdf
python3 -c "
data = open('/tmp/p611-depois.pdf', 'rb').read()
start = data.find(b'<?xpacket')
end = data.find(b'<?xpacket end')
print(data[start:end+50].decode('utf-8', errors='replace'))
"
```

Comparar campo a campo com o XML do vanilla já obtido na sonda.

Testar com caracteres acentuados no título/autor, confirmando que o XML não corrompe (o mesmo tipo de problema já corrigido para `/Info` em P538b, agora em contexto XML):

```bash
cat > /tmp/p611-acentos.typ <<'EOF'
#set document(title: "Relatório de José", author: "João Conceição")
Texto.
EOF
./target/release/typst /tmp/p611-acentos.typ /tmp/p611-acentos.pdf
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Estrutura XML confirmada contra o vanilla, não assumida.
- [ ] Stream XMP implementado e referenciado no catálogo.
- [ ] Testado com acentos, sem corrupção.
- [ ] Testado com documento sem metadados, seguindo a mesma regra de "sempre emitir" ou "condicional" já confirmada.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p611.md`, com hash do commit.
- [ ] Listas de disparidades actualizadas — este é o último item da lista original de cinco.
