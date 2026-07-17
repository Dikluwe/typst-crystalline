---
# P596 — Qual binário vanilla foi usado em P595?

> **Passo:** 596
> **Data:** 2026-07-05
> **Foco:** P595 afirma que "a versão vanilla em quarentena não suporta `page(columns:)` da mesma forma", renderizando como página de coluna única. Isto contradiz P553 e P554, que usaram um binário vanilla capaz de processar `#set page(columns: 2)` correctamente, com medições de paginação exactas. Antes de aceitar a frase de P595, confirmar qual binário exacto foi usado, e se é o mesmo dos passos anteriores.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.** Aplica-se a regra de proveniência de medição — "vanilla" não é um nome, é um binário com um hash específico, e dois relatórios não podem chamar coisas diferentes pelo mesmo nome sem se dizer isso.

---

## Verificação

```bash
ls -la lab/typst-original/target/release/typst
lab/typst-original/target/release/typst --version
git -C lab/typst-original log -1 --oneline
```

Comparar com o que P553/P554 usaram — confirmar se é o mesmo caminho de binário, a mesma versão, o mesmo commit do repositório vanilla.

### Repetir o teste exacto de P595 com este binário

```bash
cat > /tmp/p596-colunas.typ <<'EOF'
#set page(columns: 2, height: 200pt)
#lorem(30)
EOF
lab/typst-original/target/release/typst compile /tmp/p596-colunas.typ /tmp/p596-vanilla.pdf
pdfinfo /tmp/p596-vanilla.pdf | grep Pages
mutool draw -o /tmp/p596-vanilla.png -r 150 /tmp/p596-vanilla.pdf
```

Confirmar visualmente: o documento sai em duas colunas, ou numa página só, como P595 relatou?

### Se for o mesmo binário e o comportamento persistir

Confirmar se há alguma diferença na sintaxe usada — por exemplo, se P553/P554 usavam `#set page(columns: 2)` sozinho, e P595 combinou com `height: 200pt` de uma forma que quebra algo específico do vanilla.

```bash
cat > /tmp/p596-so-colunas.typ <<'EOF'
#set page(columns: 2)
#lorem(30)
EOF
lab/typst-original/target/release/typst compile /tmp/p596-so-colunas.typ /tmp/p596-so-colunas.pdf
pdfinfo /tmp/p596-so-colunas.pdf | grep Pages
```

### Critério de fecho

- [ ] Binário confirmado — mesmo caminho, versão, commit que P553/P554 usaram, ou diferente.
- [ ] Teste directo repetido, confirmando ou refutando a afirmação de P595.
- [ ] Se o binário for diferente: decidir qual é o correcto a usar daqui para a frente, e actualizar P595 se a comparação com o vanilla estava errada.
- [ ] Se o binário for o mesmo mas o comportamento mudar com `height: 200pt`: investigar essa combinação específica.

---

## Critério de fecho do passo

- [ ] Binário identificado com precisão (caminho, versão, hash).
- [ ] Comportamento confirmado com teste directo, não repetido de memória.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p596.md`.
- [ ] Se P595 estava errado sobre o vanilla: relatório de P595 corrigido, não deixado como estava.
