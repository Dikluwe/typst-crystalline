# Passo 1040 — Correcção ao Passo 1039: `gnatcov` é aberto; o bloqueio real é o rustc da AdaCore

**Tipo**: Correcção de registo — não é investigação nova, é reconciliação de um facto
verificado depois do P1039 ter fechado.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1039.

---

## O que estava errado no P1039

O relatório concluiu: *"`gnatcov` não está instalado nem disponível nos repositórios
Linux do sistema... exige a suíte GNAT Pro/Community."* Isto está incorrecto. `gnatcov`
(`GNATcoverage`) é **código aberto, licença GPLv3**
(`github.com/AdaCore/gnatcoverage`), instalável via Alire (`alire.ada.dev/crates/gnatcov`)
sem custo nem licença comercial. O P1039 só verificou `apt-cache search`, que não cobre
distribuição fora dos repositórios do sistema.

## O que estava certo, mas pela razão errada

A flag `-Ccoverage-options=mcdc` falhou de facto contra o rustc estável e nightly
disponíveis — mas não porque a flag seja inválida em geral. Confirmado na documentação
oficial do `gnatcov` para Rust:

> *"This feature is made to be compatible with the GNATPro For Rust tool suite.
> Compatibility with other Rust toolchains is not guaranteed."*

A instrumentação MC/DC para Rust (`-Ccoverage-options=mcdc`) hoje só existe no compilador
Rust próprio da AdaCore ("GNAT Pro for Rust"), produto comercial vendido por
utilizador/ano para indústria de alta integridade (aviónica, automóvel, ferroviário). A
própria AdaCore declara publicamente estar a **liderar a reintrodução** desta capacidade
no `rustc` oficial — confirma que ainda não chegou lá.

## Correcção a aplicar ao relatório do P1039

Substituir a conclusão da Fase A/D por:

> `gnatcov` em si é aberto e instalável sem custo. O bloqueio real não é o `gnatcov` — é
> que a instrumentação MC/DC do lado do compilador (`-Ccoverage-options=mcdc`) só existe
> hoje no fork comercial "GNAT Pro for Rust" da AdaCore, não no `rustc` público (estável
> ou nightly). MC/DC real para Rust, sem essa licença comercial, não está disponível hoje
> com ferramentas de terceiros — confirma-se a via de instrumentação manual/caseira como
> o caminho disponível.

## Validação

Sem alteração de código — só o texto do relatório do P1039 (e do L0/nota que o tenha
citado, se algum já existir).

---

## Resultado esperado

Registo corrigido, sem afirmar que `gnatcov` exige licença comercial quando só a
capacidade do compilador exige. A conclusão prática (via caseira) mantém-se inalterada.
