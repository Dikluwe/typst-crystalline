---
# P644 — Entradas de bibliografia omitidas sem aviso

> **Passo:** 644
> **Data:** 2026-07-09
> **Foco:** P633 confirmou dois casos: `hay_entry_to_bib_entry` (caso 5) omite entradas sem key ou sem título/autor; `bib_entry_to_hayagriva` (caso 6) omite entradas por falha de YAML ou chave ausente. P638 confirmou que o vanilla já produz erro para chave vazia, e aceita (com renderização degradada, não omissão) entradas sem título. Este passo corrige os dois últimos casos da lista de P633.
> **Tipo:** Implementação directa. Causa, localização, e comportamento do vanilla já confirmados.
> **Tamanho:** S–M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (casos confirmados), P638 (comportamento do vanilla já testado directamente).

---

## Contexto

Comportamento do vanilla, já confirmado por P638:

| Situação | Vanilla |
|---|---|
| Chave vazia | Erro: `bibliography contains entry with empty key` |
| Sem título | Aceite — renderiza de forma degradada (`[1] Author,`), não omite |
| YAML inválido | Erro propagado (`map_err(format_yaml_error)`) |

O cristalino, hoje, omite a entrada em silêncio nos três casos, em vez de: erro para chave vazia, aceitação degradada para título ausente, erro para YAML inválido.

---

## Sonda mínima

### Confirmar exactamente os dois pontos no cristalino

```bash
sed -n '135,160p' 01_core/src/engine/eval/bibliography.rs
sed -n '205,225p' 01_core/src/engine/layout/bib_csl.rs
```

Confirmar a assinatura actual das duas funções (`hay_entry_to_bib_entry`, `bib_entry_to_hayagriva`) e como os callers tratam o `None` devolvido (provavelmente `filter_map`, que descarta silenciosamente).

### Critério de fecho da sonda mínima

- [ ] Confirmado o ponto exacto onde `filter_map` (ou equivalente) descarta a entrada.
- [ ] Confirmado se as duas funções podem devolver `SourceResult<Option<T>>` (erro para chave vazia/YAML inválido, `None` só para os casos que devem mesmo ser omitidos, se algum existir) em vez de `Option<T>` simples.

---

## Implementação

### `hay_entry_to_bib_entry` (caso 5)

- Chave vazia: erro (`bibliography contains entry with empty key`, ou mensagem equivalente confirmada).
- Sem título/autor: **não omitir** — aceitar e passar adiante, deixando a renderização degradada acontecer naturalmente (como o vanilla já faz), em vez de filtrar a entrada aqui.

### `bib_entry_to_hayagriva` (caso 6)

- Falha de YAML: propagar como erro, não `None`.
- Chave ausente: erro, mesma mensagem do caso 5 se for o mesmo tipo de situação.

### Callers actualizados

Os pontos que hoje usam `filter_map` para consumir o `Option` destas funções precisam de mudar para propagar o erro (`?`), ou para colectar todos os erros antes de parar, consoante o que fizer mais sentido para uma lista de entradas bibliográficas (parar no primeiro erro, ou reportar todos de uma vez — confirmar qual o vanilla faz, se a sonda o permitir).

### Critério de fecho da implementação

- [ ] Chave vazia produz erro.
- [ ] Entrada sem título é aceite e renderizada de forma degradada, não omitida.
- [ ] YAML inválido produz erro.
- [ ] Testes de P633 (`hay_entry_to_bib_entry`, `bib_entry_to_hayagriva`, ambos via inspecção em P633, agora com teste directo) confirmando o novo comportamento.

---

## Validação

```bash
cat > /tmp/p644-chave-vazia.bib <<'EOF'
@article{,
  title = {Sem chave},
  author = {Alguém},
  year = {2024}
}
EOF
cat > /tmp/p644-chave-vazia.typ <<'EOF'
#bibliography("p644-chave-vazia.bib")
EOF
./target/release/typst /tmp/p644-chave-vazia.typ /tmp/p644.pdf
echo "Exit code: $?"
```

```bash
cat > /tmp/p644-sem-titulo.bib <<'EOF'
@article{semtitulo,
  author = {Autor Sem Título},
  year = {2024}
}
EOF
cat > /tmp/p644-sem-titulo.typ <<'EOF'
#bibliography("p644-sem-titulo.bib")
#cite(<semtitulo>)
EOF
./target/release/typst /tmp/p644-sem-titulo.typ /tmp/p644-st.pdf
pdftotext /tmp/p644-st.pdf -
```

Confirmar que a entrada sem título aparece na bibliografia, de forma degradada, não desaparece.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] Chave vazia produz erro.
- [ ] Entrada sem título aceite e renderizada, não omitida.
- [ ] YAML inválido produz erro.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p644.md`, com hash do commit.

---

## Estado da lista de P633

Com este passo, os últimos dois casos da lista original de 23 falhas silenciosas confirmadas ficam tratados. Fica só o caso 7 (grid, `unwrap_or_default`), já identificado por P638 como melhoria além do vanilla, não correcção de paridade — o critério de aceitação para esse é diferente (produzir erro claro, não bater com o vanilla, que também falha em silêncio).

E fica o débito registado por P634 e reconfirmado por P643: um passo dedicado ao parser/lexer, para propagar erros de sintaxe (`0xZZ`, escape unicode em markup) sem repetir as regressões já encontradas na primeira tentativa.
