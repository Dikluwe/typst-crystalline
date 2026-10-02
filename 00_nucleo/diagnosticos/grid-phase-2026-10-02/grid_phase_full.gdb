set pagination off
set confirm off
set print thread-events off
python
import gdb, time, json
spans=[]
class End(gdb.FinishBreakpoint):
    def __init__(self, label, started):
        super().__init__(internal=True)
        self.silent=True
        self.label=label
        self.started=started
    def stop(self):
        spans.append({"phase":self.label,"ms":(time.perf_counter_ns()-self.started)/1e6})
        return False
class Start(gdb.Breakpoint):
    def __init__(self, sym, label):
        super().__init__(sym, internal=True)
        self.silent=True
        self.label=label
    def stop(self):
        End(self.label, time.perf_counter_ns())
        return False
Start('_ZN10typst_core8compiler6layout4grid13cell_emission69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$15emit_grid_cells17h404e2c249ac40409E', 'emit')
Start('_ZN10typst_core8compiler6layout4grid13cell_emission69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$15emit_grid_cells17h7871b87e27acd52aE', 'emit')
Start('_ZN10typst_core8compiler6layout4grid13cell_emission69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$15emit_grid_cells17hf4635f07b128f7f2E', 'emit')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$17resolve_grid_rows17h611e292ee838dc60E', 'rows')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$17resolve_grid_rows17he2db668766815dbeE', 'rows')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$17resolve_grid_rows17hf442e94ebafed13cE', 'rows')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$20resolve_grid_columns17h019d0dfc3a6ed517E', 'columns')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$20resolve_grid_columns17h419c2fbbc51c52b2E', 'columns')
Start('_ZN10typst_core8compiler6layout4grid6tracks69_$LT$impl$u20$typst_core..compiler..layout..Layouter$LT$M$C$S$GT$$GT$20resolve_grid_columns17h4c33e597eb57feecE', 'columns')
def done(event):
    print("GRID_PHASES "+json.dumps(spans))
gdb.events.exited.connect(done)
end
run
quit
