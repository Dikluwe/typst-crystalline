# Passo P184F — Fecho série P184 + actualização DEBT M4-residual

Quinto e último passo de implementação P184 (após P184A
diagnóstico, P184B refinamento arm, P184C trait method,
P184D migração consumer, P184E tests E2E).
Magnitude **S**.

Passo **documental puro** — consolida série P184 num
relatório único; encerra formalmente a série; actualiza
DEBT M4-residual conforme estado de P183F (cenários A ou
B em P184A §11).

Após P184F:
- Série P184 fechada formalmente.
- DEBT M4-residual cobre apenas **C1 + C2** (não C3).
- Relatório consolidado P184A–E produzido.
- Estado pós-P184: C3 desbloqueado e validado; C1+C2
  esperam P185+ (location-aware Layouter); M9 11/11; M5/M4
  progresso 6/12 read-sites migrados.

**Pré-condição**: P184E concluído. Tests workspace 1.769
verdes; zero violations. C3 desbloqueado em pipeline real
(5 tests E2E em `p184e_figure_per_kind`).

**Restrições**:
- **Zero código tocado** em
  `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`.
- **Zero testes** novos ou modificados.
- **Zero L0s** modificados (lint não dispara — apenas
  documentação `.md` em `00_nucleo/`).
- API pública preservada.
- Output observable inalterado.

---

## Sub-passos

### .A Auditoria de estado P183F

P184A §11 antecipou dois cenários para o DEBT M4-residual
conforme ordem de execução de P183F vs P184F:

- **Cenário A**: P183F já correu antes de P184F. DEBT
  M4-residual já está aberto cobrindo C1+C2+C3. P184F
  edita o DEBT removendo C3.
- **Cenário B**: P183F ainda não correu. P184F precede;
  quando P183F correr posteriormente, abre DEBT cobrindo
  apenas C1+C2 (não C3).

Sub-passo `.A` confirma qual cenário aplica:

1. Inspeccionar `00_nucleo/diagnosticos/m1-lacunas-captura.md`
   (ou ficheiro DEBT específico se existir):
   - `grep -rn "M4-residual\|DEBT.*C1.*C2\|DEBT.*C3"
     00_nucleo/`.
   - Identificar se há entrada formal aberta para
     M4-residual.

2. Inspeccionar relatório de P183F (se existir):
   - `00_nucleo/materialization/typst-passo-183f-relatorio.md`.
   - Se ficheiro existe: cenário A.
   - Se não existe: cenário B.

3. Caso ambíguo (P183F parcialmente executado): registar
   estado factual e prosseguir conforme dados existentes.

Output: cenário identificado (A ou B) + ficheiros
relevantes localizados.

**Critério de saída**:
- Cenário A ou B confirmado.
- Localizações exactas para edits (se cenário A) ou para
  registo de pré-condição (se cenário B).

### .B Actualizar DEBT M4-residual (conforme cenário)

#### Cenário A — P183F já correu

1. Localizar entrada DEBT M4-residual em
   `m1-lacunas-captura.md` ou ficheiro dedicado.

2. Editar:
   - Remover C3 da lista de consumers cobertos.
   - Manter C1 + C2.
   - Actualizar texto:
     - "DEBT M4-residual cobre **C1 + C2**" (era
       C1+C2+C3).
     - "C3 fechado em P184D + P184E" (referência
       cruzada).
   - Magnitude actualizada: trabalho restante =
     desbloqueio location-aware (C1+C2). Custo M+
     (cross-cutting M6+).

3. Hash em branco aguarda recálculo se ficheiro tiver
   formato L0.

#### Cenário B — P183F ainda não correu

1. Adicionar entrada preventiva em
   `m1-lacunas-captura.md` (se aplicável) ou em
   localização equivalente:
   - Registar que P184 fechou C3 antes de P183F.
   - Documentar que quando P183F correr, deve abrir
     DEBT cobrindo apenas C1+C2.
   - Cross-reference: relatório P184F.

2. Não editar P183F (passo independente que ainda não
   correu).

**Critério de saída**:
- DEBT actualizado ou nota preventiva registada.
- Texto reflecte fecho de C3.
- Magnitude e cobertura literal actualizadas.

### .C Escrever relatório consolidado P184

1. Criar
   `00_nucleo/materialization/typst-passo-184-relatorio-consolidado.md`
   com 9 secções (padrão P181J / P182F):

   - §1 Resumo executivo + pipeline final desbloqueio
     C3.
   - §2 Sub-passos materializados (tabela métricas A–E).
   - §3 Decisões arquitecturais (6 cláusulas P184A
     fechadas).
   - §4 Achados não-triviais durante execução
     (P184D §1 "dead code em produção" ratificado;
     inversão Introspector vs fallback legacy;
     ajuste empírico em P184E `.E`).
   - §5 Estado final M9 (inalterado 11/11) e M5/M4
     (6/12 read-sites; +1 vs P183).
   - §6 Estado final lacunas (3 abertas; #3 separada).
   - §7 Pendências cumulativas + janela compat M6.
   - §8 Próximos passos sugeridos (P185A
     location-aware Layouter; P186 Equation
     locatable; P187 migrar C1; P188 migrar C2).
   - §9 Conclusão.

2. Sem L0 novo; sem alteração de tests; sem ADR; sem
   DEBT novo.

**Critério de saída**:
- Relatório consolidado existe em
  `00_nucleo/materialization/`.
- 9 secções presentes.
- Dados consistentes com relatórios individuais P184A–E.

### .D Verificação estrutural

1. `cargo check --workspace` passa (sem código tocado).
2. `cargo test --workspace --lib` passa: **1.769**
   inalterado vs P184E.
3. `crystalline-lint .` zero violations.
4. Relatório consolidado existe com 9 secções.
5. DEBT M4-residual actualizado (cenário A) ou nota
   preventiva registada (cenário B).
6. Sem código de produção tocado.
7. Sem L0 modificado.
8. Sem tests modificados.

### .E Encerramento

P184F é o passo de encerramento. Após `.D` concluído, a
série P184 está formalmente fechada.

Estado projectado pós-P184F:

- **P184 série**: A ✅ B ✅ C ✅ D ✅ E ✅ **F ✅**.
  Fechada.
- **C3 fechado e validado**.
- **DEBT M4-residual**: cobre apenas C1+C2 (cenário A)
  ou nota preventiva registada (cenário B).
- **M9**: 11/11 (inalterado).
- **M5/M4 progresso**: 6/12 read-sites migrados.
- **Lacunas**: 3 abertas (#1, #2, #3 — todas adiadas
  intencionalmente; nenhuma bloqueia M5/M6/M7/M8).
- **Próximo substantivo**: P185A (diagnóstico location-aware
  Layouter para desbloquear C1+C2).
- **Padrão diagnóstico-primeiro**: 9ª aplicação
  (131A/132A/140A/148/154A/181A/182A/183A/184A).

---

## Critério de conclusão

Todas em conjunto:

1. `.A` confirmou cenário (A ou B).
2. DEBT M4-residual actualizado ou nota preventiva
   registada.
3. Relatório consolidado P184 (9 secções) escrito.
4. Verificações `.D` passam (8/8).
5. Sem código de produção tocado.
6. Sem L0 modificado.
7. Sem tests modificados.
8. Sem ADR; sem DEBT novo.

---

## O que pode sair errado

- **DEBT M4-residual em formato inesperado**: cláusula
  gate trivial — auditar ficheiro actual antes de editar.
- **Cenário ambíguo** (P183F parcialmente executado):
  cláusula gate trivial — registar estado factual e
  prosseguir conforme dados existentes.
- **Conflito entre P183F e P184F na mesma entrada**: se
  P183F abriu entrada cobrindo C1+C2+C3 e P184F precisa
  de remover C3, garantir que edit não desfaz alterações
  de P183F que cobrem C1+C2. Cláusula gate trivial — ler
  ficheiro inteiro antes de editar.
- **Relatório consolidado já existe** (improvável): se
  alguém criou rascunho, reescrever ou incrementar.
- **Linter dispara em ficheiros `.md`**: improvável; lint
  cobre L0 prompts em `prompts/`, não outros documentos.

---

## Notas operacionais

- **Tamanho**: S puro. ~150-250 LOC em
  `00_nucleo/materialization/` + ~15 LOC em
  `00_nucleo/diagnosticos/` (DEBT update).
- **Sem código tocado**.
- **Sem testes**.
- **Sem ADR; sem DEBT novo**.
- **Padrão replicado**: P181J + P182F (consolidador
  documental).
- **Cláusula gate trivial**: aplicável a formato do DEBT,
  cenário A vs B, conflitos de edit.
- **Sem cláusula gate substancial**.
- **Após P184F**, foco passa para **P185A**: diagnóstico
  location-aware Layouter. Pendência P182E §5.2
  finalmente atacada.
- **Estado consolidado da série P184**: 4 passos de
  implementação (B–E) + 1 documental (A) + 1 documental
  (F) = 6 sub-passos. Magnitude agregada S (não S-M
  como P183A previa — execução foi mais limpa porque
  C3 não tinha bloqueios cruzados como C1/C2).
- **Inversão observable face a P182**: P182 fechou
  lacuna #4 mas Introspector ficou redundante (fallback
  legacy mutável é o caminho funcional). P184 fecha C3
  com Introspector como caminho funcional (legacy é
  dead code). Isto ratifica a regra dos 2 eixos — eixo
  1 OK + eixo 2 atendido = consumer realmente migrado.
