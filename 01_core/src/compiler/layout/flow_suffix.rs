//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/flow_suffix.md
//! @prompt-hash bc5ca7d1
//! @layer L1
//! @updated 2026-09-25
//!
//! Aceite, rollback e replay da unidade de flow posterior ao marker.

use super::metrics::FontMetrics;
use crate::entities::image_sizer::ImageSizer;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Confirma a transação ativa antes de fechar a região; uma tentativa
    /// rejeitada não pode materializar página antes do rollback.
    pub(super) fn accept_flow_suffix_before_region_transition(&mut self) -> bool {
        // Uma transição ordinária aceita a unidade corrente antes de fechar a
        // região. Uma tentativa já rejeitada não pode materializar páginas:
        // a transação restaurará a cauda antes do avanço ordinário.
        if self.flow_suffix_transaction_active {
            if self.flow_suffix_transaction_rejected {
                return false;
            }
            self.flow_suffix_transaction_active = false;
            self.flow_suffix_checkpoint = None;
            self.flow_suffix_replay.clear();
        }
        true
    }

    /// P1292-D v11 — compõe a primeira unidade posterior ao marker como uma
    /// transação regional. A tentativa inicial usa o caminho de layout
    /// ordinário e não conhece o variant do conteúdo. Se `flush_line`
    /// detectar overflow causado pela área reduzida, restaura todo o estado
    /// visível, fecha a região contendo apenas o prefixo confirmado e repete
    /// exactamente a mesma ocorrência na região seguinte.
    pub(super) fn layout_flow_suffix_transaction(
        &mut self,
        content: &crate::entities::content::Content,
    ) {
        self.flow_suffix_replay.push(content.clone());
        self.flow_suffix_dispatch_depth += 1;
        self.layout_content_once(content);
        self.flow_suffix_dispatch_depth -= 1;

        // O Block posterior pode terminar sem rejeição imediata. A decisão
        // só se torna observável quando uma inserção diferida do próprio
        // sufixo aparece. Place calcula o prefixo admissível sem o realizar;
        // se alguma ocorrência nova não cabe, a unidade inteira é rejeitada.
        let checkpoint_float_len = self
            .flow_suffix_checkpoint
            .as_ref()
            .map(|checkpoint| checkpoint.floats_pending_len)
            .unwrap_or(0);
        if self.floats_pending.len() > checkpoint_float_len {
            let fitting = super::place::fitting_float_prefix_len(self);
            if fitting < self.floats_pending.len() {
                self.flow_suffix_transaction_rejected = true;
            }
        }

        let rejected = self.flow_suffix_transaction_rejected;
        if !rejected {
            // Uma ocorrência diferida nova que cabe torna a fronteira
            // estável. Sem inserção nova, uma ocorrência de lookahead é
            // suficiente para decidir a unidade de flow sem manter estado
            // especulativo através de fronteiras documentais arbitrárias.
            if self.floats_pending.len() > checkpoint_float_len
                || self.flow_suffix_replay.len() >= 2
            {
                self.flow_suffix_transaction_active = false;
                self.flow_suffix_checkpoint = None;
                self.flow_suffix_replay.clear();
            }
            return;
        }

        let checkpoint = self
            .flow_suffix_checkpoint
            .take()
            .expect("transação de sufixo activa deve possuir checkpoint");
        let replay = std::mem::take(&mut self.flow_suffix_replay);
        self.flow_suffix_transaction_active = false;
        self.flow_suffix_transaction_rejected = false;
        self.restore_flow_suffix_checkpoint(checkpoint);
        self.new_page();
        self.flow_suffix_dispatch_depth = 1;
        for occurrence in &replay {
            self.layout_content_once(occurrence);
        }
        self.flow_suffix_dispatch_depth = 0;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1504_flow_suffix_possui_replay() {
        assert!(include_str!("flow_suffix.rs").contains("restore_flow_suffix_checkpoint"));
    }
}
