# L0 — Passo 1078: Divergência `Seção`/`Secção` em Português — Achado Lateral do P1073 (Origem P788)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (todo documento em
português com heading numerado é afectado). **Requer confirmação do dono antes
de codificar.**

**Base**: achado isolado no P1073 — `translations/pt.txt:5` do vanilla define
`heading = Seção` (sem `c`); o cristalino usa `Secção` (com `c`), fixado desde o
P788.

---

## 1. Não é necessariamente um bug simples — verificar variante regional antes

`Secção` (com `c`) é a grafia pré-Acordo Ortográfico de 1990 (Portugal);
`Seção` (sem `c`) é a grafia pós-acordo (Brasil, e Portugal desde a adopção do
acordo). Isto não é erro de digitação — são duas normas ortográficas
legítimas do português, cada uma correcta no seu contexto.

**Antes de assumir que `Secção` está errado e deve virar `Seção`**: confirmar se
o vanilla distingue `pt` (Brasil) de `pt-PT`/variante equivalente (Portugal) nas
suas tabelas de tradução, ou se só tem um único `pt` (citado no P1073 como
`translations/pt.txt`, sem qualificação regional visível na citação já feita).

```bash
find lab/typst-original -iname "*.txt" -path "*translations*" | grep -i pt
```

Se existirem `pt.txt` e algo como `pt-PT.txt` separados, o cristalino pode
estar simplesmente a resolver para a tabela errada (usar sempre `Secção`
independente da variante pedida), o que é bug real, mas de natureza diferente
(selecção de tabela, não valor da tabela). Se só existir um `pt.txt` com
`Seção`, então o cristalino diverge mesmo do único padrão oficial, e `Secção`
não tem justificação — corrigir directamente.

## 2. Ler o código real antes de corrigir

Pedir ou localizar onde `Secção` está fixado — provavelmente
`01_core/src/compiler/layout/references.rs` (mesmo mecanismo
`default_supplement_for_key` já tocado no P1073) ou em `heading.rs`, dependendo
de onde o P788 escreveu o valor originalmente.

## 3. Verificar se `Secção` aparece só num lugar ou em vários

O P788 (citado em `compiler/layout.md`, secção anterior desta conversa) já
tinha testes legacy com a expectativa `"Secção 1"` explicitamente codificada.
Se essa mudança for feita, os testes que fixam esse valor precisam de
actualização — não deixar testes desactualizados a validar o valor errado
depois da correcção do código de produção.

```bash
grep -rn "Secção" 01_core/src/
```

## 4. Medição

Documento em português com heading numerado, `@h` e output directo, comparar
contra o vanilla (confirmando primeiro, via §1, qual é o `pt` real de
referência a usar).

## 5. Critérios de verificação

1. Heading numerado em português → texto correcto conforme §1 confirmar.
2. `@h` (referência cruzada) — mesmo valor, sem regressão da correcção do P1073
   (que já tratava `@h` como caso já-correcto antes deste achado aparecer).
3. Testes antigos do P788 actualizados, não deixados a validar valor obsoleto.
4. `crystalline-lint .` — 0 erros.
5. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1 respondido — variante regional confirmada, não presumida.
- Código real localizado e citado (não só "algures em references.rs").
- Todos os usos de `Secção`/`Seção` no código de produção reconciliados, não só
  o ponto citado no P1073.
