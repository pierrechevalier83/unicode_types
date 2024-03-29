
/// An enum to represent all characters in the ArabicPresentationFormsA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArabicPresentationFormsA {
    /// \u{fb50}: 'ﭐ'
    ArabicLetterAlefWaslaIsolatedForm,
    /// \u{fb51}: 'ﭑ'
    ArabicLetterAlefWaslaFinalForm,
    /// \u{fb52}: 'ﭒ'
    ArabicLetterBeehIsolatedForm,
    /// \u{fb53}: 'ﭓ'
    ArabicLetterBeehFinalForm,
    /// \u{fb54}: 'ﭔ'
    ArabicLetterBeehInitialForm,
    /// \u{fb55}: 'ﭕ'
    ArabicLetterBeehMedialForm,
    /// \u{fb56}: 'ﭖ'
    ArabicLetterPehIsolatedForm,
    /// \u{fb57}: 'ﭗ'
    ArabicLetterPehFinalForm,
    /// \u{fb58}: 'ﭘ'
    ArabicLetterPehInitialForm,
    /// \u{fb59}: 'ﭙ'
    ArabicLetterPehMedialForm,
    /// \u{fb5a}: 'ﭚ'
    ArabicLetterBehehIsolatedForm,
    /// \u{fb5b}: 'ﭛ'
    ArabicLetterBehehFinalForm,
    /// \u{fb5c}: 'ﭜ'
    ArabicLetterBehehInitialForm,
    /// \u{fb5d}: 'ﭝ'
    ArabicLetterBehehMedialForm,
    /// \u{fb5e}: 'ﭞ'
    ArabicLetterTtehehIsolatedForm,
    /// \u{fb5f}: 'ﭟ'
    ArabicLetterTtehehFinalForm,
    /// \u{fb60}: 'ﭠ'
    ArabicLetterTtehehInitialForm,
    /// \u{fb61}: 'ﭡ'
    ArabicLetterTtehehMedialForm,
    /// \u{fb62}: 'ﭢ'
    ArabicLetterTehehIsolatedForm,
    /// \u{fb63}: 'ﭣ'
    ArabicLetterTehehFinalForm,
    /// \u{fb64}: 'ﭤ'
    ArabicLetterTehehInitialForm,
    /// \u{fb65}: 'ﭥ'
    ArabicLetterTehehMedialForm,
    /// \u{fb66}: 'ﭦ'
    ArabicLetterTtehIsolatedForm,
    /// \u{fb67}: 'ﭧ'
    ArabicLetterTtehFinalForm,
    /// \u{fb68}: 'ﭨ'
    ArabicLetterTtehInitialForm,
    /// \u{fb69}: 'ﭩ'
    ArabicLetterTtehMedialForm,
    /// \u{fb6a}: 'ﭪ'
    ArabicLetterVehIsolatedForm,
    /// \u{fb6b}: 'ﭫ'
    ArabicLetterVehFinalForm,
    /// \u{fb6c}: 'ﭬ'
    ArabicLetterVehInitialForm,
    /// \u{fb6d}: 'ﭭ'
    ArabicLetterVehMedialForm,
    /// \u{fb6e}: 'ﭮ'
    ArabicLetterPehehIsolatedForm,
    /// \u{fb6f}: 'ﭯ'
    ArabicLetterPehehFinalForm,
    /// \u{fb70}: 'ﭰ'
    ArabicLetterPehehInitialForm,
    /// \u{fb71}: 'ﭱ'
    ArabicLetterPehehMedialForm,
    /// \u{fb72}: 'ﭲ'
    ArabicLetterDyehIsolatedForm,
    /// \u{fb73}: 'ﭳ'
    ArabicLetterDyehFinalForm,
    /// \u{fb74}: 'ﭴ'
    ArabicLetterDyehInitialForm,
    /// \u{fb75}: 'ﭵ'
    ArabicLetterDyehMedialForm,
    /// \u{fb76}: 'ﭶ'
    ArabicLetterNyehIsolatedForm,
    /// \u{fb77}: 'ﭷ'
    ArabicLetterNyehFinalForm,
    /// \u{fb78}: 'ﭸ'
    ArabicLetterNyehInitialForm,
    /// \u{fb79}: 'ﭹ'
    ArabicLetterNyehMedialForm,
    /// \u{fb7a}: 'ﭺ'
    ArabicLetterTchehIsolatedForm,
    /// \u{fb7b}: 'ﭻ'
    ArabicLetterTchehFinalForm,
    /// \u{fb7c}: 'ﭼ'
    ArabicLetterTchehInitialForm,
    /// \u{fb7d}: 'ﭽ'
    ArabicLetterTchehMedialForm,
    /// \u{fb7e}: 'ﭾ'
    ArabicLetterTchehehIsolatedForm,
    /// \u{fb7f}: 'ﭿ'
    ArabicLetterTchehehFinalForm,
    /// \u{fb80}: 'ﮀ'
    ArabicLetterTchehehInitialForm,
    /// \u{fb81}: 'ﮁ'
    ArabicLetterTchehehMedialForm,
    /// \u{fb82}: 'ﮂ'
    ArabicLetterDdahalIsolatedForm,
    /// \u{fb83}: 'ﮃ'
    ArabicLetterDdahalFinalForm,
    /// \u{fb84}: 'ﮄ'
    ArabicLetterDahalIsolatedForm,
    /// \u{fb85}: 'ﮅ'
    ArabicLetterDahalFinalForm,
    /// \u{fb86}: 'ﮆ'
    ArabicLetterDulIsolatedForm,
    /// \u{fb87}: 'ﮇ'
    ArabicLetterDulFinalForm,
    /// \u{fb88}: 'ﮈ'
    ArabicLetterDdalIsolatedForm,
    /// \u{fb89}: 'ﮉ'
    ArabicLetterDdalFinalForm,
    /// \u{fb8a}: 'ﮊ'
    ArabicLetterJehIsolatedForm,
    /// \u{fb8b}: 'ﮋ'
    ArabicLetterJehFinalForm,
    /// \u{fb8c}: 'ﮌ'
    ArabicLetterRrehIsolatedForm,
    /// \u{fb8d}: 'ﮍ'
    ArabicLetterRrehFinalForm,
    /// \u{fb8e}: 'ﮎ'
    ArabicLetterKehehIsolatedForm,
    /// \u{fb8f}: 'ﮏ'
    ArabicLetterKehehFinalForm,
    /// \u{fb90}: 'ﮐ'
    ArabicLetterKehehInitialForm,
    /// \u{fb91}: 'ﮑ'
    ArabicLetterKehehMedialForm,
    /// \u{fb92}: 'ﮒ'
    ArabicLetterGafIsolatedForm,
    /// \u{fb93}: 'ﮓ'
    ArabicLetterGafFinalForm,
    /// \u{fb94}: 'ﮔ'
    ArabicLetterGafInitialForm,
    /// \u{fb95}: 'ﮕ'
    ArabicLetterGafMedialForm,
    /// \u{fb96}: 'ﮖ'
    ArabicLetterGuehIsolatedForm,
    /// \u{fb97}: 'ﮗ'
    ArabicLetterGuehFinalForm,
    /// \u{fb98}: 'ﮘ'
    ArabicLetterGuehInitialForm,
    /// \u{fb99}: 'ﮙ'
    ArabicLetterGuehMedialForm,
    /// \u{fb9a}: 'ﮚ'
    ArabicLetterNgoehIsolatedForm,
    /// \u{fb9b}: 'ﮛ'
    ArabicLetterNgoehFinalForm,
    /// \u{fb9c}: 'ﮜ'
    ArabicLetterNgoehInitialForm,
    /// \u{fb9d}: 'ﮝ'
    ArabicLetterNgoehMedialForm,
    /// \u{fb9e}: 'ﮞ'
    ArabicLetterNoonGhunnaIsolatedForm,
    /// \u{fb9f}: 'ﮟ'
    ArabicLetterNoonGhunnaFinalForm,
    /// \u{fba0}: 'ﮠ'
    ArabicLetterRnoonIsolatedForm,
    /// \u{fba1}: 'ﮡ'
    ArabicLetterRnoonFinalForm,
    /// \u{fba2}: 'ﮢ'
    ArabicLetterRnoonInitialForm,
    /// \u{fba3}: 'ﮣ'
    ArabicLetterRnoonMedialForm,
    /// \u{fba4}: 'ﮤ'
    ArabicLetterHehWithYehAboveIsolatedForm,
    /// \u{fba5}: 'ﮥ'
    ArabicLetterHehWithYehAboveFinalForm,
    /// \u{fba6}: 'ﮦ'
    ArabicLetterHehGoalIsolatedForm,
    /// \u{fba7}: 'ﮧ'
    ArabicLetterHehGoalFinalForm,
    /// \u{fba8}: 'ﮨ'
    ArabicLetterHehGoalInitialForm,
    /// \u{fba9}: 'ﮩ'
    ArabicLetterHehGoalMedialForm,
    /// \u{fbaa}: 'ﮪ'
    ArabicLetterHehDoachashmeeIsolatedForm,
    /// \u{fbab}: 'ﮫ'
    ArabicLetterHehDoachashmeeFinalForm,
    /// \u{fbac}: 'ﮬ'
    ArabicLetterHehDoachashmeeInitialForm,
    /// \u{fbad}: 'ﮭ'
    ArabicLetterHehDoachashmeeMedialForm,
    /// \u{fbae}: 'ﮮ'
    ArabicLetterYehBarreeIsolatedForm,
    /// \u{fbaf}: 'ﮯ'
    ArabicLetterYehBarreeFinalForm,
    /// \u{fbb0}: 'ﮰ'
    ArabicLetterYehBarreeWithHamzaAboveIsolatedForm,
    /// \u{fbb1}: 'ﮱ'
    ArabicLetterYehBarreeWithHamzaAboveFinalForm,
    /// \u{fbb2}: '﮲'
    ArabicSymbolDotAbove,
    /// \u{fbb3}: '﮳'
    ArabicSymbolDotBelow,
    /// \u{fbb4}: '﮴'
    ArabicSymbolTwoDotsAbove,
    /// \u{fbb5}: '﮵'
    ArabicSymbolTwoDotsBelow,
    /// \u{fbb6}: '﮶'
    ArabicSymbolThreeDotsAbove,
    /// \u{fbb7}: '﮷'
    ArabicSymbolThreeDotsBelow,
    /// \u{fbb8}: '﮸'
    ArabicSymbolThreeDotsPointingDownwardsAbove,
    /// \u{fbb9}: '﮹'
    ArabicSymbolThreeDotsPointingDownwardsBelow,
    /// \u{fbba}: '﮺'
    ArabicSymbolFourDotsAbove,
    /// \u{fbbb}: '﮻'
    ArabicSymbolFourDotsBelow,
    /// \u{fbbc}: '﮼'
    ArabicSymbolDoubleVerticalBarBelow,
    /// \u{fbbd}: '﮽'
    ArabicSymbolTwoDotsVerticallyAbove,
    /// \u{fbbe}: '﮾'
    ArabicSymbolTwoDotsVerticallyBelow,
    /// \u{fbbf}: '﮿'
    ArabicSymbolRing,
    /// \u{fbc0}: '﯀'
    ArabicSymbolSmallTahAbove,
    /// \u{fbc1}: '﯁'
    ArabicSymbolSmallTahBelow,
    /// \u{fbd3}: 'ﯓ'
    ArabicLetterNgIsolatedForm,
    /// \u{fbd4}: 'ﯔ'
    ArabicLetterNgFinalForm,
    /// \u{fbd5}: 'ﯕ'
    ArabicLetterNgInitialForm,
    /// \u{fbd6}: 'ﯖ'
    ArabicLetterNgMedialForm,
    /// \u{fbd7}: 'ﯗ'
    ArabicLetterUIsolatedForm,
    /// \u{fbd8}: 'ﯘ'
    ArabicLetterUFinalForm,
    /// \u{fbd9}: 'ﯙ'
    ArabicLetterOeIsolatedForm,
    /// \u{fbda}: 'ﯚ'
    ArabicLetterOeFinalForm,
    /// \u{fbdb}: 'ﯛ'
    ArabicLetterYuIsolatedForm,
    /// \u{fbdc}: 'ﯜ'
    ArabicLetterYuFinalForm,
    /// \u{fbdd}: 'ﯝ'
    ArabicLetterUWithHamzaAboveIsolatedForm,
    /// \u{fbde}: 'ﯞ'
    ArabicLetterVeIsolatedForm,
    /// \u{fbdf}: 'ﯟ'
    ArabicLetterVeFinalForm,
    /// \u{fbe0}: 'ﯠ'
    ArabicLetterKirghizOeIsolatedForm,
    /// \u{fbe1}: 'ﯡ'
    ArabicLetterKirghizOeFinalForm,
    /// \u{fbe2}: 'ﯢ'
    ArabicLetterKirghizYuIsolatedForm,
    /// \u{fbe3}: 'ﯣ'
    ArabicLetterKirghizYuFinalForm,
    /// \u{fbe4}: 'ﯤ'
    ArabicLetterEIsolatedForm,
    /// \u{fbe5}: 'ﯥ'
    ArabicLetterEFinalForm,
    /// \u{fbe6}: 'ﯦ'
    ArabicLetterEInitialForm,
    /// \u{fbe7}: 'ﯧ'
    ArabicLetterEMedialForm,
    /// \u{fbe8}: 'ﯨ'
    ArabicLetterUighurKazakhKirghizAlefMaksuraInitialForm,
    /// \u{fbe9}: 'ﯩ'
    ArabicLetterUighurKazakhKirghizAlefMaksuraMedialForm,
    /// \u{fbea}: 'ﯪ'
    ArabicLigatureYehWithHamzaAboveWithAlefIsolatedForm,
    /// \u{fbeb}: 'ﯫ'
    ArabicLigatureYehWithHamzaAboveWithAlefFinalForm,
    /// \u{fbec}: 'ﯬ'
    ArabicLigatureYehWithHamzaAboveWithAeIsolatedForm,
    /// \u{fbed}: 'ﯭ'
    ArabicLigatureYehWithHamzaAboveWithAeFinalForm,
    /// \u{fbee}: 'ﯮ'
    ArabicLigatureYehWithHamzaAboveWithWawIsolatedForm,
    /// \u{fbef}: 'ﯯ'
    ArabicLigatureYehWithHamzaAboveWithWawFinalForm,
    /// \u{fbf0}: 'ﯰ'
    ArabicLigatureYehWithHamzaAboveWithUIsolatedForm,
    /// \u{fbf1}: 'ﯱ'
    ArabicLigatureYehWithHamzaAboveWithUFinalForm,
    /// \u{fbf2}: 'ﯲ'
    ArabicLigatureYehWithHamzaAboveWithOeIsolatedForm,
    /// \u{fbf3}: 'ﯳ'
    ArabicLigatureYehWithHamzaAboveWithOeFinalForm,
    /// \u{fbf4}: 'ﯴ'
    ArabicLigatureYehWithHamzaAboveWithYuIsolatedForm,
    /// \u{fbf5}: 'ﯵ'
    ArabicLigatureYehWithHamzaAboveWithYuFinalForm,
    /// \u{fbf6}: 'ﯶ'
    ArabicLigatureYehWithHamzaAboveWithEIsolatedForm,
    /// \u{fbf7}: 'ﯷ'
    ArabicLigatureYehWithHamzaAboveWithEFinalForm,
    /// \u{fbf8}: 'ﯸ'
    ArabicLigatureYehWithHamzaAboveWithEInitialForm,
    /// \u{fbf9}: 'ﯹ'
    ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraIsolatedForm,
    /// \u{fbfa}: 'ﯺ'
    ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraFinalForm,
    /// \u{fbfb}: 'ﯻ'
    ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraInitialForm,
    /// \u{fbfc}: 'ﯼ'
    ArabicLetterFarsiYehIsolatedForm,
    /// \u{fbfd}: 'ﯽ'
    ArabicLetterFarsiYehFinalForm,
    /// \u{fbfe}: 'ﯾ'
    ArabicLetterFarsiYehInitialForm,
    /// \u{fbff}: 'ﯿ'
    ArabicLetterFarsiYehMedialForm,
    /// \u{fc00}: 'ﰀ'
    ArabicLigatureYehWithHamzaAboveWithJeemIsolatedForm,
    /// \u{fc01}: 'ﰁ'
    ArabicLigatureYehWithHamzaAboveWithHahIsolatedForm,
    /// \u{fc02}: 'ﰂ'
    ArabicLigatureYehWithHamzaAboveWithMeemIsolatedForm,
    /// \u{fc03}: 'ﰃ'
    ArabicLigatureYehWithHamzaAboveWithAlefMaksuraIsolatedForm,
    /// \u{fc04}: 'ﰄ'
    ArabicLigatureYehWithHamzaAboveWithYehIsolatedForm,
    /// \u{fc05}: 'ﰅ'
    ArabicLigatureBehWithJeemIsolatedForm,
    /// \u{fc06}: 'ﰆ'
    ArabicLigatureBehWithHahIsolatedForm,
    /// \u{fc07}: 'ﰇ'
    ArabicLigatureBehWithKhahIsolatedForm,
    /// \u{fc08}: 'ﰈ'
    ArabicLigatureBehWithMeemIsolatedForm,
    /// \u{fc09}: 'ﰉ'
    ArabicLigatureBehWithAlefMaksuraIsolatedForm,
    /// \u{fc0a}: 'ﰊ'
    ArabicLigatureBehWithYehIsolatedForm,
    /// \u{fc0b}: 'ﰋ'
    ArabicLigatureTehWithJeemIsolatedForm,
    /// \u{fc0c}: 'ﰌ'
    ArabicLigatureTehWithHahIsolatedForm,
    /// \u{fc0d}: 'ﰍ'
    ArabicLigatureTehWithKhahIsolatedForm,
    /// \u{fc0e}: 'ﰎ'
    ArabicLigatureTehWithMeemIsolatedForm,
    /// \u{fc0f}: 'ﰏ'
    ArabicLigatureTehWithAlefMaksuraIsolatedForm,
    /// \u{fc10}: 'ﰐ'
    ArabicLigatureTehWithYehIsolatedForm,
    /// \u{fc11}: 'ﰑ'
    ArabicLigatureThehWithJeemIsolatedForm,
    /// \u{fc12}: 'ﰒ'
    ArabicLigatureThehWithMeemIsolatedForm,
    /// \u{fc13}: 'ﰓ'
    ArabicLigatureThehWithAlefMaksuraIsolatedForm,
    /// \u{fc14}: 'ﰔ'
    ArabicLigatureThehWithYehIsolatedForm,
    /// \u{fc15}: 'ﰕ'
    ArabicLigatureJeemWithHahIsolatedForm,
    /// \u{fc16}: 'ﰖ'
    ArabicLigatureJeemWithMeemIsolatedForm,
    /// \u{fc17}: 'ﰗ'
    ArabicLigatureHahWithJeemIsolatedForm,
    /// \u{fc18}: 'ﰘ'
    ArabicLigatureHahWithMeemIsolatedForm,
    /// \u{fc19}: 'ﰙ'
    ArabicLigatureKhahWithJeemIsolatedForm,
    /// \u{fc1a}: 'ﰚ'
    ArabicLigatureKhahWithHahIsolatedForm,
    /// \u{fc1b}: 'ﰛ'
    ArabicLigatureKhahWithMeemIsolatedForm,
    /// \u{fc1c}: 'ﰜ'
    ArabicLigatureSeenWithJeemIsolatedForm,
    /// \u{fc1d}: 'ﰝ'
    ArabicLigatureSeenWithHahIsolatedForm,
    /// \u{fc1e}: 'ﰞ'
    ArabicLigatureSeenWithKhahIsolatedForm,
    /// \u{fc1f}: 'ﰟ'
    ArabicLigatureSeenWithMeemIsolatedForm,
    /// \u{fc20}: 'ﰠ'
    ArabicLigatureSadWithHahIsolatedForm,
    /// \u{fc21}: 'ﰡ'
    ArabicLigatureSadWithMeemIsolatedForm,
    /// \u{fc22}: 'ﰢ'
    ArabicLigatureDadWithJeemIsolatedForm,
    /// \u{fc23}: 'ﰣ'
    ArabicLigatureDadWithHahIsolatedForm,
    /// \u{fc24}: 'ﰤ'
    ArabicLigatureDadWithKhahIsolatedForm,
    /// \u{fc25}: 'ﰥ'
    ArabicLigatureDadWithMeemIsolatedForm,
    /// \u{fc26}: 'ﰦ'
    ArabicLigatureTahWithHahIsolatedForm,
    /// \u{fc27}: 'ﰧ'
    ArabicLigatureTahWithMeemIsolatedForm,
    /// \u{fc28}: 'ﰨ'
    ArabicLigatureZahWithMeemIsolatedForm,
    /// \u{fc29}: 'ﰩ'
    ArabicLigatureAinWithJeemIsolatedForm,
    /// \u{fc2a}: 'ﰪ'
    ArabicLigatureAinWithMeemIsolatedForm,
    /// \u{fc2b}: 'ﰫ'
    ArabicLigatureGhainWithJeemIsolatedForm,
    /// \u{fc2c}: 'ﰬ'
    ArabicLigatureGhainWithMeemIsolatedForm,
    /// \u{fc2d}: 'ﰭ'
    ArabicLigatureFehWithJeemIsolatedForm,
    /// \u{fc2e}: 'ﰮ'
    ArabicLigatureFehWithHahIsolatedForm,
    /// \u{fc2f}: 'ﰯ'
    ArabicLigatureFehWithKhahIsolatedForm,
    /// \u{fc30}: 'ﰰ'
    ArabicLigatureFehWithMeemIsolatedForm,
    /// \u{fc31}: 'ﰱ'
    ArabicLigatureFehWithAlefMaksuraIsolatedForm,
    /// \u{fc32}: 'ﰲ'
    ArabicLigatureFehWithYehIsolatedForm,
    /// \u{fc33}: 'ﰳ'
    ArabicLigatureQafWithHahIsolatedForm,
    /// \u{fc34}: 'ﰴ'
    ArabicLigatureQafWithMeemIsolatedForm,
    /// \u{fc35}: 'ﰵ'
    ArabicLigatureQafWithAlefMaksuraIsolatedForm,
    /// \u{fc36}: 'ﰶ'
    ArabicLigatureQafWithYehIsolatedForm,
    /// \u{fc37}: 'ﰷ'
    ArabicLigatureKafWithAlefIsolatedForm,
    /// \u{fc38}: 'ﰸ'
    ArabicLigatureKafWithJeemIsolatedForm,
    /// \u{fc39}: 'ﰹ'
    ArabicLigatureKafWithHahIsolatedForm,
    /// \u{fc3a}: 'ﰺ'
    ArabicLigatureKafWithKhahIsolatedForm,
    /// \u{fc3b}: 'ﰻ'
    ArabicLigatureKafWithLamIsolatedForm,
    /// \u{fc3c}: 'ﰼ'
    ArabicLigatureKafWithMeemIsolatedForm,
    /// \u{fc3d}: 'ﰽ'
    ArabicLigatureKafWithAlefMaksuraIsolatedForm,
    /// \u{fc3e}: 'ﰾ'
    ArabicLigatureKafWithYehIsolatedForm,
    /// \u{fc3f}: 'ﰿ'
    ArabicLigatureLamWithJeemIsolatedForm,
    /// \u{fc40}: 'ﱀ'
    ArabicLigatureLamWithHahIsolatedForm,
    /// \u{fc41}: 'ﱁ'
    ArabicLigatureLamWithKhahIsolatedForm,
    /// \u{fc42}: 'ﱂ'
    ArabicLigatureLamWithMeemIsolatedForm,
    /// \u{fc43}: 'ﱃ'
    ArabicLigatureLamWithAlefMaksuraIsolatedForm,
    /// \u{fc44}: 'ﱄ'
    ArabicLigatureLamWithYehIsolatedForm,
    /// \u{fc45}: 'ﱅ'
    ArabicLigatureMeemWithJeemIsolatedForm,
    /// \u{fc46}: 'ﱆ'
    ArabicLigatureMeemWithHahIsolatedForm,
    /// \u{fc47}: 'ﱇ'
    ArabicLigatureMeemWithKhahIsolatedForm,
    /// \u{fc48}: 'ﱈ'
    ArabicLigatureMeemWithMeemIsolatedForm,
    /// \u{fc49}: 'ﱉ'
    ArabicLigatureMeemWithAlefMaksuraIsolatedForm,
    /// \u{fc4a}: 'ﱊ'
    ArabicLigatureMeemWithYehIsolatedForm,
    /// \u{fc4b}: 'ﱋ'
    ArabicLigatureNoonWithJeemIsolatedForm,
    /// \u{fc4c}: 'ﱌ'
    ArabicLigatureNoonWithHahIsolatedForm,
    /// \u{fc4d}: 'ﱍ'
    ArabicLigatureNoonWithKhahIsolatedForm,
    /// \u{fc4e}: 'ﱎ'
    ArabicLigatureNoonWithMeemIsolatedForm,
    /// \u{fc4f}: 'ﱏ'
    ArabicLigatureNoonWithAlefMaksuraIsolatedForm,
    /// \u{fc50}: 'ﱐ'
    ArabicLigatureNoonWithYehIsolatedForm,
    /// \u{fc51}: 'ﱑ'
    ArabicLigatureHehWithJeemIsolatedForm,
    /// \u{fc52}: 'ﱒ'
    ArabicLigatureHehWithMeemIsolatedForm,
    /// \u{fc53}: 'ﱓ'
    ArabicLigatureHehWithAlefMaksuraIsolatedForm,
    /// \u{fc54}: 'ﱔ'
    ArabicLigatureHehWithYehIsolatedForm,
    /// \u{fc55}: 'ﱕ'
    ArabicLigatureYehWithJeemIsolatedForm,
    /// \u{fc56}: 'ﱖ'
    ArabicLigatureYehWithHahIsolatedForm,
    /// \u{fc57}: 'ﱗ'
    ArabicLigatureYehWithKhahIsolatedForm,
    /// \u{fc58}: 'ﱘ'
    ArabicLigatureYehWithMeemIsolatedForm,
    /// \u{fc59}: 'ﱙ'
    ArabicLigatureYehWithAlefMaksuraIsolatedForm,
    /// \u{fc5a}: 'ﱚ'
    ArabicLigatureYehWithYehIsolatedForm,
    /// \u{fc5b}: 'ﱛ'
    ArabicLigatureThalWithSuperscriptAlefIsolatedForm,
    /// \u{fc5c}: 'ﱜ'
    ArabicLigatureRehWithSuperscriptAlefIsolatedForm,
    /// \u{fc5d}: 'ﱝ'
    ArabicLigatureAlefMaksuraWithSuperscriptAlefIsolatedForm,
    /// \u{fc5e}: 'ﱞ'
    ArabicLigatureShaddaWithDammatanIsolatedForm,
    /// \u{fc5f}: 'ﱟ'
    ArabicLigatureShaddaWithKasratanIsolatedForm,
    /// \u{fc60}: 'ﱠ'
    ArabicLigatureShaddaWithFathaIsolatedForm,
    /// \u{fc61}: 'ﱡ'
    ArabicLigatureShaddaWithDammaIsolatedForm,
    /// \u{fc62}: 'ﱢ'
    ArabicLigatureShaddaWithKasraIsolatedForm,
    /// \u{fc63}: 'ﱣ'
    ArabicLigatureShaddaWithSuperscriptAlefIsolatedForm,
    /// \u{fc64}: 'ﱤ'
    ArabicLigatureYehWithHamzaAboveWithRehFinalForm,
    /// \u{fc65}: 'ﱥ'
    ArabicLigatureYehWithHamzaAboveWithZainFinalForm,
    /// \u{fc66}: 'ﱦ'
    ArabicLigatureYehWithHamzaAboveWithMeemFinalForm,
    /// \u{fc67}: 'ﱧ'
    ArabicLigatureYehWithHamzaAboveWithNoonFinalForm,
    /// \u{fc68}: 'ﱨ'
    ArabicLigatureYehWithHamzaAboveWithAlefMaksuraFinalForm,
    /// \u{fc69}: 'ﱩ'
    ArabicLigatureYehWithHamzaAboveWithYehFinalForm,
    /// \u{fc6a}: 'ﱪ'
    ArabicLigatureBehWithRehFinalForm,
    /// \u{fc6b}: 'ﱫ'
    ArabicLigatureBehWithZainFinalForm,
    /// \u{fc6c}: 'ﱬ'
    ArabicLigatureBehWithMeemFinalForm,
    /// \u{fc6d}: 'ﱭ'
    ArabicLigatureBehWithNoonFinalForm,
    /// \u{fc6e}: 'ﱮ'
    ArabicLigatureBehWithAlefMaksuraFinalForm,
    /// \u{fc6f}: 'ﱯ'
    ArabicLigatureBehWithYehFinalForm,
    /// \u{fc70}: 'ﱰ'
    ArabicLigatureTehWithRehFinalForm,
    /// \u{fc71}: 'ﱱ'
    ArabicLigatureTehWithZainFinalForm,
    /// \u{fc72}: 'ﱲ'
    ArabicLigatureTehWithMeemFinalForm,
    /// \u{fc73}: 'ﱳ'
    ArabicLigatureTehWithNoonFinalForm,
    /// \u{fc74}: 'ﱴ'
    ArabicLigatureTehWithAlefMaksuraFinalForm,
    /// \u{fc75}: 'ﱵ'
    ArabicLigatureTehWithYehFinalForm,
    /// \u{fc76}: 'ﱶ'
    ArabicLigatureThehWithRehFinalForm,
    /// \u{fc77}: 'ﱷ'
    ArabicLigatureThehWithZainFinalForm,
    /// \u{fc78}: 'ﱸ'
    ArabicLigatureThehWithMeemFinalForm,
    /// \u{fc79}: 'ﱹ'
    ArabicLigatureThehWithNoonFinalForm,
    /// \u{fc7a}: 'ﱺ'
    ArabicLigatureThehWithAlefMaksuraFinalForm,
    /// \u{fc7b}: 'ﱻ'
    ArabicLigatureThehWithYehFinalForm,
    /// \u{fc7c}: 'ﱼ'
    ArabicLigatureFehWithAlefMaksuraFinalForm,
    /// \u{fc7d}: 'ﱽ'
    ArabicLigatureFehWithYehFinalForm,
    /// \u{fc7e}: 'ﱾ'
    ArabicLigatureQafWithAlefMaksuraFinalForm,
    /// \u{fc7f}: 'ﱿ'
    ArabicLigatureQafWithYehFinalForm,
    /// \u{fc80}: 'ﲀ'
    ArabicLigatureKafWithAlefFinalForm,
    /// \u{fc81}: 'ﲁ'
    ArabicLigatureKafWithLamFinalForm,
    /// \u{fc82}: 'ﲂ'
    ArabicLigatureKafWithMeemFinalForm,
    /// \u{fc83}: 'ﲃ'
    ArabicLigatureKafWithAlefMaksuraFinalForm,
    /// \u{fc84}: 'ﲄ'
    ArabicLigatureKafWithYehFinalForm,
    /// \u{fc85}: 'ﲅ'
    ArabicLigatureLamWithMeemFinalForm,
    /// \u{fc86}: 'ﲆ'
    ArabicLigatureLamWithAlefMaksuraFinalForm,
    /// \u{fc87}: 'ﲇ'
    ArabicLigatureLamWithYehFinalForm,
    /// \u{fc88}: 'ﲈ'
    ArabicLigatureMeemWithAlefFinalForm,
    /// \u{fc89}: 'ﲉ'
    ArabicLigatureMeemWithMeemFinalForm,
    /// \u{fc8a}: 'ﲊ'
    ArabicLigatureNoonWithRehFinalForm,
    /// \u{fc8b}: 'ﲋ'
    ArabicLigatureNoonWithZainFinalForm,
    /// \u{fc8c}: 'ﲌ'
    ArabicLigatureNoonWithMeemFinalForm,
    /// \u{fc8d}: 'ﲍ'
    ArabicLigatureNoonWithNoonFinalForm,
    /// \u{fc8e}: 'ﲎ'
    ArabicLigatureNoonWithAlefMaksuraFinalForm,
    /// \u{fc8f}: 'ﲏ'
    ArabicLigatureNoonWithYehFinalForm,
    /// \u{fc90}: 'ﲐ'
    ArabicLigatureAlefMaksuraWithSuperscriptAlefFinalForm,
    /// \u{fc91}: 'ﲑ'
    ArabicLigatureYehWithRehFinalForm,
    /// \u{fc92}: 'ﲒ'
    ArabicLigatureYehWithZainFinalForm,
    /// \u{fc93}: 'ﲓ'
    ArabicLigatureYehWithMeemFinalForm,
    /// \u{fc94}: 'ﲔ'
    ArabicLigatureYehWithNoonFinalForm,
    /// \u{fc95}: 'ﲕ'
    ArabicLigatureYehWithAlefMaksuraFinalForm,
    /// \u{fc96}: 'ﲖ'
    ArabicLigatureYehWithYehFinalForm,
    /// \u{fc97}: 'ﲗ'
    ArabicLigatureYehWithHamzaAboveWithJeemInitialForm,
    /// \u{fc98}: 'ﲘ'
    ArabicLigatureYehWithHamzaAboveWithHahInitialForm,
    /// \u{fc99}: 'ﲙ'
    ArabicLigatureYehWithHamzaAboveWithKhahInitialForm,
    /// \u{fc9a}: 'ﲚ'
    ArabicLigatureYehWithHamzaAboveWithMeemInitialForm,
    /// \u{fc9b}: 'ﲛ'
    ArabicLigatureYehWithHamzaAboveWithHehInitialForm,
    /// \u{fc9c}: 'ﲜ'
    ArabicLigatureBehWithJeemInitialForm,
    /// \u{fc9d}: 'ﲝ'
    ArabicLigatureBehWithHahInitialForm,
    /// \u{fc9e}: 'ﲞ'
    ArabicLigatureBehWithKhahInitialForm,
    /// \u{fc9f}: 'ﲟ'
    ArabicLigatureBehWithMeemInitialForm,
    /// \u{fca0}: 'ﲠ'
    ArabicLigatureBehWithHehInitialForm,
    /// \u{fca1}: 'ﲡ'
    ArabicLigatureTehWithJeemInitialForm,
    /// \u{fca2}: 'ﲢ'
    ArabicLigatureTehWithHahInitialForm,
    /// \u{fca3}: 'ﲣ'
    ArabicLigatureTehWithKhahInitialForm,
    /// \u{fca4}: 'ﲤ'
    ArabicLigatureTehWithMeemInitialForm,
    /// \u{fca5}: 'ﲥ'
    ArabicLigatureTehWithHehInitialForm,
    /// \u{fca6}: 'ﲦ'
    ArabicLigatureThehWithMeemInitialForm,
    /// \u{fca7}: 'ﲧ'
    ArabicLigatureJeemWithHahInitialForm,
    /// \u{fca8}: 'ﲨ'
    ArabicLigatureJeemWithMeemInitialForm,
    /// \u{fca9}: 'ﲩ'
    ArabicLigatureHahWithJeemInitialForm,
    /// \u{fcaa}: 'ﲪ'
    ArabicLigatureHahWithMeemInitialForm,
    /// \u{fcab}: 'ﲫ'
    ArabicLigatureKhahWithJeemInitialForm,
    /// \u{fcac}: 'ﲬ'
    ArabicLigatureKhahWithMeemInitialForm,
    /// \u{fcad}: 'ﲭ'
    ArabicLigatureSeenWithJeemInitialForm,
    /// \u{fcae}: 'ﲮ'
    ArabicLigatureSeenWithHahInitialForm,
    /// \u{fcaf}: 'ﲯ'
    ArabicLigatureSeenWithKhahInitialForm,
    /// \u{fcb0}: 'ﲰ'
    ArabicLigatureSeenWithMeemInitialForm,
    /// \u{fcb1}: 'ﲱ'
    ArabicLigatureSadWithHahInitialForm,
    /// \u{fcb2}: 'ﲲ'
    ArabicLigatureSadWithKhahInitialForm,
    /// \u{fcb3}: 'ﲳ'
    ArabicLigatureSadWithMeemInitialForm,
    /// \u{fcb4}: 'ﲴ'
    ArabicLigatureDadWithJeemInitialForm,
    /// \u{fcb5}: 'ﲵ'
    ArabicLigatureDadWithHahInitialForm,
    /// \u{fcb6}: 'ﲶ'
    ArabicLigatureDadWithKhahInitialForm,
    /// \u{fcb7}: 'ﲷ'
    ArabicLigatureDadWithMeemInitialForm,
    /// \u{fcb8}: 'ﲸ'
    ArabicLigatureTahWithHahInitialForm,
    /// \u{fcb9}: 'ﲹ'
    ArabicLigatureZahWithMeemInitialForm,
    /// \u{fcba}: 'ﲺ'
    ArabicLigatureAinWithJeemInitialForm,
    /// \u{fcbb}: 'ﲻ'
    ArabicLigatureAinWithMeemInitialForm,
    /// \u{fcbc}: 'ﲼ'
    ArabicLigatureGhainWithJeemInitialForm,
    /// \u{fcbd}: 'ﲽ'
    ArabicLigatureGhainWithMeemInitialForm,
    /// \u{fcbe}: 'ﲾ'
    ArabicLigatureFehWithJeemInitialForm,
    /// \u{fcbf}: 'ﲿ'
    ArabicLigatureFehWithHahInitialForm,
    /// \u{fcc0}: 'ﳀ'
    ArabicLigatureFehWithKhahInitialForm,
    /// \u{fcc1}: 'ﳁ'
    ArabicLigatureFehWithMeemInitialForm,
    /// \u{fcc2}: 'ﳂ'
    ArabicLigatureQafWithHahInitialForm,
    /// \u{fcc3}: 'ﳃ'
    ArabicLigatureQafWithMeemInitialForm,
    /// \u{fcc4}: 'ﳄ'
    ArabicLigatureKafWithJeemInitialForm,
    /// \u{fcc5}: 'ﳅ'
    ArabicLigatureKafWithHahInitialForm,
    /// \u{fcc6}: 'ﳆ'
    ArabicLigatureKafWithKhahInitialForm,
    /// \u{fcc7}: 'ﳇ'
    ArabicLigatureKafWithLamInitialForm,
    /// \u{fcc8}: 'ﳈ'
    ArabicLigatureKafWithMeemInitialForm,
    /// \u{fcc9}: 'ﳉ'
    ArabicLigatureLamWithJeemInitialForm,
    /// \u{fcca}: 'ﳊ'
    ArabicLigatureLamWithHahInitialForm,
    /// \u{fccb}: 'ﳋ'
    ArabicLigatureLamWithKhahInitialForm,
    /// \u{fccc}: 'ﳌ'
    ArabicLigatureLamWithMeemInitialForm,
    /// \u{fccd}: 'ﳍ'
    ArabicLigatureLamWithHehInitialForm,
    /// \u{fcce}: 'ﳎ'
    ArabicLigatureMeemWithJeemInitialForm,
    /// \u{fccf}: 'ﳏ'
    ArabicLigatureMeemWithHahInitialForm,
    /// \u{fcd0}: 'ﳐ'
    ArabicLigatureMeemWithKhahInitialForm,
    /// \u{fcd1}: 'ﳑ'
    ArabicLigatureMeemWithMeemInitialForm,
    /// \u{fcd2}: 'ﳒ'
    ArabicLigatureNoonWithJeemInitialForm,
    /// \u{fcd3}: 'ﳓ'
    ArabicLigatureNoonWithHahInitialForm,
    /// \u{fcd4}: 'ﳔ'
    ArabicLigatureNoonWithKhahInitialForm,
    /// \u{fcd5}: 'ﳕ'
    ArabicLigatureNoonWithMeemInitialForm,
    /// \u{fcd6}: 'ﳖ'
    ArabicLigatureNoonWithHehInitialForm,
    /// \u{fcd7}: 'ﳗ'
    ArabicLigatureHehWithJeemInitialForm,
    /// \u{fcd8}: 'ﳘ'
    ArabicLigatureHehWithMeemInitialForm,
    /// \u{fcd9}: 'ﳙ'
    ArabicLigatureHehWithSuperscriptAlefInitialForm,
    /// \u{fcda}: 'ﳚ'
    ArabicLigatureYehWithJeemInitialForm,
    /// \u{fcdb}: 'ﳛ'
    ArabicLigatureYehWithHahInitialForm,
    /// \u{fcdc}: 'ﳜ'
    ArabicLigatureYehWithKhahInitialForm,
    /// \u{fcdd}: 'ﳝ'
    ArabicLigatureYehWithMeemInitialForm,
    /// \u{fcde}: 'ﳞ'
    ArabicLigatureYehWithHehInitialForm,
    /// \u{fcdf}: 'ﳟ'
    ArabicLigatureYehWithHamzaAboveWithMeemMedialForm,
    /// \u{fce0}: 'ﳠ'
    ArabicLigatureYehWithHamzaAboveWithHehMedialForm,
    /// \u{fce1}: 'ﳡ'
    ArabicLigatureBehWithMeemMedialForm,
    /// \u{fce2}: 'ﳢ'
    ArabicLigatureBehWithHehMedialForm,
    /// \u{fce3}: 'ﳣ'
    ArabicLigatureTehWithMeemMedialForm,
    /// \u{fce4}: 'ﳤ'
    ArabicLigatureTehWithHehMedialForm,
    /// \u{fce5}: 'ﳥ'
    ArabicLigatureThehWithMeemMedialForm,
    /// \u{fce6}: 'ﳦ'
    ArabicLigatureThehWithHehMedialForm,
    /// \u{fce7}: 'ﳧ'
    ArabicLigatureSeenWithMeemMedialForm,
    /// \u{fce8}: 'ﳨ'
    ArabicLigatureSeenWithHehMedialForm,
    /// \u{fce9}: 'ﳩ'
    ArabicLigatureSheenWithMeemMedialForm,
    /// \u{fcea}: 'ﳪ'
    ArabicLigatureSheenWithHehMedialForm,
    /// \u{fceb}: 'ﳫ'
    ArabicLigatureKafWithLamMedialForm,
    /// \u{fcec}: 'ﳬ'
    ArabicLigatureKafWithMeemMedialForm,
    /// \u{fced}: 'ﳭ'
    ArabicLigatureLamWithMeemMedialForm,
    /// \u{fcee}: 'ﳮ'
    ArabicLigatureNoonWithMeemMedialForm,
    /// \u{fcef}: 'ﳯ'
    ArabicLigatureNoonWithHehMedialForm,
    /// \u{fcf0}: 'ﳰ'
    ArabicLigatureYehWithMeemMedialForm,
    /// \u{fcf1}: 'ﳱ'
    ArabicLigatureYehWithHehMedialForm,
    /// \u{fcf2}: 'ﳲ'
    ArabicLigatureShaddaWithFathaMedialForm,
    /// \u{fcf3}: 'ﳳ'
    ArabicLigatureShaddaWithDammaMedialForm,
    /// \u{fcf4}: 'ﳴ'
    ArabicLigatureShaddaWithKasraMedialForm,
    /// \u{fcf5}: 'ﳵ'
    ArabicLigatureTahWithAlefMaksuraIsolatedForm,
    /// \u{fcf6}: 'ﳶ'
    ArabicLigatureTahWithYehIsolatedForm,
    /// \u{fcf7}: 'ﳷ'
    ArabicLigatureAinWithAlefMaksuraIsolatedForm,
    /// \u{fcf8}: 'ﳸ'
    ArabicLigatureAinWithYehIsolatedForm,
    /// \u{fcf9}: 'ﳹ'
    ArabicLigatureGhainWithAlefMaksuraIsolatedForm,
    /// \u{fcfa}: 'ﳺ'
    ArabicLigatureGhainWithYehIsolatedForm,
    /// \u{fcfb}: 'ﳻ'
    ArabicLigatureSeenWithAlefMaksuraIsolatedForm,
    /// \u{fcfc}: 'ﳼ'
    ArabicLigatureSeenWithYehIsolatedForm,
    /// \u{fcfd}: 'ﳽ'
    ArabicLigatureSheenWithAlefMaksuraIsolatedForm,
    /// \u{fcfe}: 'ﳾ'
    ArabicLigatureSheenWithYehIsolatedForm,
    /// \u{fcff}: 'ﳿ'
    ArabicLigatureHahWithAlefMaksuraIsolatedForm,
    /// \u{fd00}: 'ﴀ'
    ArabicLigatureHahWithYehIsolatedForm,
    /// \u{fd01}: 'ﴁ'
    ArabicLigatureJeemWithAlefMaksuraIsolatedForm,
    /// \u{fd02}: 'ﴂ'
    ArabicLigatureJeemWithYehIsolatedForm,
    /// \u{fd03}: 'ﴃ'
    ArabicLigatureKhahWithAlefMaksuraIsolatedForm,
    /// \u{fd04}: 'ﴄ'
    ArabicLigatureKhahWithYehIsolatedForm,
    /// \u{fd05}: 'ﴅ'
    ArabicLigatureSadWithAlefMaksuraIsolatedForm,
    /// \u{fd06}: 'ﴆ'
    ArabicLigatureSadWithYehIsolatedForm,
    /// \u{fd07}: 'ﴇ'
    ArabicLigatureDadWithAlefMaksuraIsolatedForm,
    /// \u{fd08}: 'ﴈ'
    ArabicLigatureDadWithYehIsolatedForm,
    /// \u{fd09}: 'ﴉ'
    ArabicLigatureSheenWithJeemIsolatedForm,
    /// \u{fd0a}: 'ﴊ'
    ArabicLigatureSheenWithHahIsolatedForm,
    /// \u{fd0b}: 'ﴋ'
    ArabicLigatureSheenWithKhahIsolatedForm,
    /// \u{fd0c}: 'ﴌ'
    ArabicLigatureSheenWithMeemIsolatedForm,
    /// \u{fd0d}: 'ﴍ'
    ArabicLigatureSheenWithRehIsolatedForm,
    /// \u{fd0e}: 'ﴎ'
    ArabicLigatureSeenWithRehIsolatedForm,
    /// \u{fd0f}: 'ﴏ'
    ArabicLigatureSadWithRehIsolatedForm,
    /// \u{fd10}: 'ﴐ'
    ArabicLigatureDadWithRehIsolatedForm,
    /// \u{fd11}: 'ﴑ'
    ArabicLigatureTahWithAlefMaksuraFinalForm,
    /// \u{fd12}: 'ﴒ'
    ArabicLigatureTahWithYehFinalForm,
    /// \u{fd13}: 'ﴓ'
    ArabicLigatureAinWithAlefMaksuraFinalForm,
    /// \u{fd14}: 'ﴔ'
    ArabicLigatureAinWithYehFinalForm,
    /// \u{fd15}: 'ﴕ'
    ArabicLigatureGhainWithAlefMaksuraFinalForm,
    /// \u{fd16}: 'ﴖ'
    ArabicLigatureGhainWithYehFinalForm,
    /// \u{fd17}: 'ﴗ'
    ArabicLigatureSeenWithAlefMaksuraFinalForm,
    /// \u{fd18}: 'ﴘ'
    ArabicLigatureSeenWithYehFinalForm,
    /// \u{fd19}: 'ﴙ'
    ArabicLigatureSheenWithAlefMaksuraFinalForm,
    /// \u{fd1a}: 'ﴚ'
    ArabicLigatureSheenWithYehFinalForm,
    /// \u{fd1b}: 'ﴛ'
    ArabicLigatureHahWithAlefMaksuraFinalForm,
    /// \u{fd1c}: 'ﴜ'
    ArabicLigatureHahWithYehFinalForm,
    /// \u{fd1d}: 'ﴝ'
    ArabicLigatureJeemWithAlefMaksuraFinalForm,
    /// \u{fd1e}: 'ﴞ'
    ArabicLigatureJeemWithYehFinalForm,
    /// \u{fd1f}: 'ﴟ'
    ArabicLigatureKhahWithAlefMaksuraFinalForm,
    /// \u{fd20}: 'ﴠ'
    ArabicLigatureKhahWithYehFinalForm,
    /// \u{fd21}: 'ﴡ'
    ArabicLigatureSadWithAlefMaksuraFinalForm,
    /// \u{fd22}: 'ﴢ'
    ArabicLigatureSadWithYehFinalForm,
    /// \u{fd23}: 'ﴣ'
    ArabicLigatureDadWithAlefMaksuraFinalForm,
    /// \u{fd24}: 'ﴤ'
    ArabicLigatureDadWithYehFinalForm,
    /// \u{fd25}: 'ﴥ'
    ArabicLigatureSheenWithJeemFinalForm,
    /// \u{fd26}: 'ﴦ'
    ArabicLigatureSheenWithHahFinalForm,
    /// \u{fd27}: 'ﴧ'
    ArabicLigatureSheenWithKhahFinalForm,
    /// \u{fd28}: 'ﴨ'
    ArabicLigatureSheenWithMeemFinalForm,
    /// \u{fd29}: 'ﴩ'
    ArabicLigatureSheenWithRehFinalForm,
    /// \u{fd2a}: 'ﴪ'
    ArabicLigatureSeenWithRehFinalForm,
    /// \u{fd2b}: 'ﴫ'
    ArabicLigatureSadWithRehFinalForm,
    /// \u{fd2c}: 'ﴬ'
    ArabicLigatureDadWithRehFinalForm,
    /// \u{fd2d}: 'ﴭ'
    ArabicLigatureSheenWithJeemInitialForm,
    /// \u{fd2e}: 'ﴮ'
    ArabicLigatureSheenWithHahInitialForm,
    /// \u{fd2f}: 'ﴯ'
    ArabicLigatureSheenWithKhahInitialForm,
    /// \u{fd30}: 'ﴰ'
    ArabicLigatureSheenWithMeemInitialForm,
    /// \u{fd31}: 'ﴱ'
    ArabicLigatureSeenWithHehInitialForm,
    /// \u{fd32}: 'ﴲ'
    ArabicLigatureSheenWithHehInitialForm,
    /// \u{fd33}: 'ﴳ'
    ArabicLigatureTahWithMeemInitialForm,
    /// \u{fd34}: 'ﴴ'
    ArabicLigatureSeenWithJeemMedialForm,
    /// \u{fd35}: 'ﴵ'
    ArabicLigatureSeenWithHahMedialForm,
    /// \u{fd36}: 'ﴶ'
    ArabicLigatureSeenWithKhahMedialForm,
    /// \u{fd37}: 'ﴷ'
    ArabicLigatureSheenWithJeemMedialForm,
    /// \u{fd38}: 'ﴸ'
    ArabicLigatureSheenWithHahMedialForm,
    /// \u{fd39}: 'ﴹ'
    ArabicLigatureSheenWithKhahMedialForm,
    /// \u{fd3a}: 'ﴺ'
    ArabicLigatureTahWithMeemMedialForm,
    /// \u{fd3b}: 'ﴻ'
    ArabicLigatureZahWithMeemMedialForm,
    /// \u{fd3c}: 'ﴼ'
    ArabicLigatureAlefWithFathatanFinalForm,
    /// \u{fd3d}: 'ﴽ'
    ArabicLigatureAlefWithFathatanIsolatedForm,
    /// \u{fd3e}: '﴾'
    OrnateLeftParenthesis,
    /// \u{fd3f}: '﴿'
    OrnateRightParenthesis,
    /// \u{fd50}: 'ﵐ'
    ArabicLigatureTehWithJeemWithMeemInitialForm,
    /// \u{fd51}: 'ﵑ'
    ArabicLigatureTehWithHahWithJeemFinalForm,
    /// \u{fd52}: 'ﵒ'
    ArabicLigatureTehWithHahWithJeemInitialForm,
    /// \u{fd53}: 'ﵓ'
    ArabicLigatureTehWithHahWithMeemInitialForm,
    /// \u{fd54}: 'ﵔ'
    ArabicLigatureTehWithKhahWithMeemInitialForm,
    /// \u{fd55}: 'ﵕ'
    ArabicLigatureTehWithMeemWithJeemInitialForm,
    /// \u{fd56}: 'ﵖ'
    ArabicLigatureTehWithMeemWithHahInitialForm,
    /// \u{fd57}: 'ﵗ'
    ArabicLigatureTehWithMeemWithKhahInitialForm,
    /// \u{fd58}: 'ﵘ'
    ArabicLigatureJeemWithMeemWithHahFinalForm,
    /// \u{fd59}: 'ﵙ'
    ArabicLigatureJeemWithMeemWithHahInitialForm,
    /// \u{fd5a}: 'ﵚ'
    ArabicLigatureHahWithMeemWithYehFinalForm,
    /// \u{fd5b}: 'ﵛ'
    ArabicLigatureHahWithMeemWithAlefMaksuraFinalForm,
    /// \u{fd5c}: 'ﵜ'
    ArabicLigatureSeenWithHahWithJeemInitialForm,
    /// \u{fd5d}: 'ﵝ'
    ArabicLigatureSeenWithJeemWithHahInitialForm,
    /// \u{fd5e}: 'ﵞ'
    ArabicLigatureSeenWithJeemWithAlefMaksuraFinalForm,
    /// \u{fd5f}: 'ﵟ'
    ArabicLigatureSeenWithMeemWithHahFinalForm,
    /// \u{fd60}: 'ﵠ'
    ArabicLigatureSeenWithMeemWithHahInitialForm,
    /// \u{fd61}: 'ﵡ'
    ArabicLigatureSeenWithMeemWithJeemInitialForm,
    /// \u{fd62}: 'ﵢ'
    ArabicLigatureSeenWithMeemWithMeemFinalForm,
    /// \u{fd63}: 'ﵣ'
    ArabicLigatureSeenWithMeemWithMeemInitialForm,
    /// \u{fd64}: 'ﵤ'
    ArabicLigatureSadWithHahWithHahFinalForm,
    /// \u{fd65}: 'ﵥ'
    ArabicLigatureSadWithHahWithHahInitialForm,
    /// \u{fd66}: 'ﵦ'
    ArabicLigatureSadWithMeemWithMeemFinalForm,
    /// \u{fd67}: 'ﵧ'
    ArabicLigatureSheenWithHahWithMeemFinalForm,
    /// \u{fd68}: 'ﵨ'
    ArabicLigatureSheenWithHahWithMeemInitialForm,
    /// \u{fd69}: 'ﵩ'
    ArabicLigatureSheenWithJeemWithYehFinalForm,
    /// \u{fd6a}: 'ﵪ'
    ArabicLigatureSheenWithMeemWithKhahFinalForm,
    /// \u{fd6b}: 'ﵫ'
    ArabicLigatureSheenWithMeemWithKhahInitialForm,
    /// \u{fd6c}: 'ﵬ'
    ArabicLigatureSheenWithMeemWithMeemFinalForm,
    /// \u{fd6d}: 'ﵭ'
    ArabicLigatureSheenWithMeemWithMeemInitialForm,
    /// \u{fd6e}: 'ﵮ'
    ArabicLigatureDadWithHahWithAlefMaksuraFinalForm,
    /// \u{fd6f}: 'ﵯ'
    ArabicLigatureDadWithKhahWithMeemFinalForm,
    /// \u{fd70}: 'ﵰ'
    ArabicLigatureDadWithKhahWithMeemInitialForm,
    /// \u{fd71}: 'ﵱ'
    ArabicLigatureTahWithMeemWithHahFinalForm,
    /// \u{fd72}: 'ﵲ'
    ArabicLigatureTahWithMeemWithHahInitialForm,
    /// \u{fd73}: 'ﵳ'
    ArabicLigatureTahWithMeemWithMeemInitialForm,
    /// \u{fd74}: 'ﵴ'
    ArabicLigatureTahWithMeemWithYehFinalForm,
    /// \u{fd75}: 'ﵵ'
    ArabicLigatureAinWithJeemWithMeemFinalForm,
    /// \u{fd76}: 'ﵶ'
    ArabicLigatureAinWithMeemWithMeemFinalForm,
    /// \u{fd77}: 'ﵷ'
    ArabicLigatureAinWithMeemWithMeemInitialForm,
    /// \u{fd78}: 'ﵸ'
    ArabicLigatureAinWithMeemWithAlefMaksuraFinalForm,
    /// \u{fd79}: 'ﵹ'
    ArabicLigatureGhainWithMeemWithMeemFinalForm,
    /// \u{fd7a}: 'ﵺ'
    ArabicLigatureGhainWithMeemWithYehFinalForm,
    /// \u{fd7b}: 'ﵻ'
    ArabicLigatureGhainWithMeemWithAlefMaksuraFinalForm,
    /// \u{fd7c}: 'ﵼ'
    ArabicLigatureFehWithKhahWithMeemFinalForm,
    /// \u{fd7d}: 'ﵽ'
    ArabicLigatureFehWithKhahWithMeemInitialForm,
    /// \u{fd7e}: 'ﵾ'
    ArabicLigatureQafWithMeemWithHahFinalForm,
    /// \u{fd7f}: 'ﵿ'
    ArabicLigatureQafWithMeemWithMeemFinalForm,
    /// \u{fd80}: 'ﶀ'
    ArabicLigatureLamWithHahWithMeemFinalForm,
    /// \u{fd81}: 'ﶁ'
    ArabicLigatureLamWithHahWithYehFinalForm,
    /// \u{fd82}: 'ﶂ'
    ArabicLigatureLamWithHahWithAlefMaksuraFinalForm,
    /// \u{fd83}: 'ﶃ'
    ArabicLigatureLamWithJeemWithJeemInitialForm,
    /// \u{fd84}: 'ﶄ'
    ArabicLigatureLamWithJeemWithJeemFinalForm,
    /// \u{fd85}: 'ﶅ'
    ArabicLigatureLamWithKhahWithMeemFinalForm,
    /// \u{fd86}: 'ﶆ'
    ArabicLigatureLamWithKhahWithMeemInitialForm,
    /// \u{fd87}: 'ﶇ'
    ArabicLigatureLamWithMeemWithHahFinalForm,
    /// \u{fd88}: 'ﶈ'
    ArabicLigatureLamWithMeemWithHahInitialForm,
    /// \u{fd89}: 'ﶉ'
    ArabicLigatureMeemWithHahWithJeemInitialForm,
    /// \u{fd8a}: 'ﶊ'
    ArabicLigatureMeemWithHahWithMeemInitialForm,
    /// \u{fd8b}: 'ﶋ'
    ArabicLigatureMeemWithHahWithYehFinalForm,
    /// \u{fd8c}: 'ﶌ'
    ArabicLigatureMeemWithJeemWithHahInitialForm,
    /// \u{fd8d}: 'ﶍ'
    ArabicLigatureMeemWithJeemWithMeemInitialForm,
    /// \u{fd8e}: 'ﶎ'
    ArabicLigatureMeemWithKhahWithJeemInitialForm,
    /// \u{fd8f}: 'ﶏ'
    ArabicLigatureMeemWithKhahWithMeemInitialForm,
    /// \u{fd92}: 'ﶒ'
    ArabicLigatureMeemWithJeemWithKhahInitialForm,
    /// \u{fd93}: 'ﶓ'
    ArabicLigatureHehWithMeemWithJeemInitialForm,
    /// \u{fd94}: 'ﶔ'
    ArabicLigatureHehWithMeemWithMeemInitialForm,
    /// \u{fd95}: 'ﶕ'
    ArabicLigatureNoonWithHahWithMeemInitialForm,
    /// \u{fd96}: 'ﶖ'
    ArabicLigatureNoonWithHahWithAlefMaksuraFinalForm,
    /// \u{fd97}: 'ﶗ'
    ArabicLigatureNoonWithJeemWithMeemFinalForm,
    /// \u{fd98}: 'ﶘ'
    ArabicLigatureNoonWithJeemWithMeemInitialForm,
    /// \u{fd99}: 'ﶙ'
    ArabicLigatureNoonWithJeemWithAlefMaksuraFinalForm,
    /// \u{fd9a}: 'ﶚ'
    ArabicLigatureNoonWithMeemWithYehFinalForm,
    /// \u{fd9b}: 'ﶛ'
    ArabicLigatureNoonWithMeemWithAlefMaksuraFinalForm,
    /// \u{fd9c}: 'ﶜ'
    ArabicLigatureYehWithMeemWithMeemFinalForm,
    /// \u{fd9d}: 'ﶝ'
    ArabicLigatureYehWithMeemWithMeemInitialForm,
    /// \u{fd9e}: 'ﶞ'
    ArabicLigatureBehWithKhahWithYehFinalForm,
    /// \u{fd9f}: 'ﶟ'
    ArabicLigatureTehWithJeemWithYehFinalForm,
    /// \u{fda0}: 'ﶠ'
    ArabicLigatureTehWithJeemWithAlefMaksuraFinalForm,
    /// \u{fda1}: 'ﶡ'
    ArabicLigatureTehWithKhahWithYehFinalForm,
    /// \u{fda2}: 'ﶢ'
    ArabicLigatureTehWithKhahWithAlefMaksuraFinalForm,
    /// \u{fda3}: 'ﶣ'
    ArabicLigatureTehWithMeemWithYehFinalForm,
    /// \u{fda4}: 'ﶤ'
    ArabicLigatureTehWithMeemWithAlefMaksuraFinalForm,
    /// \u{fda5}: 'ﶥ'
    ArabicLigatureJeemWithMeemWithYehFinalForm,
    /// \u{fda6}: 'ﶦ'
    ArabicLigatureJeemWithHahWithAlefMaksuraFinalForm,
    /// \u{fda7}: 'ﶧ'
    ArabicLigatureJeemWithMeemWithAlefMaksuraFinalForm,
    /// \u{fda8}: 'ﶨ'
    ArabicLigatureSeenWithKhahWithAlefMaksuraFinalForm,
    /// \u{fda9}: 'ﶩ'
    ArabicLigatureSadWithHahWithYehFinalForm,
    /// \u{fdaa}: 'ﶪ'
    ArabicLigatureSheenWithHahWithYehFinalForm,
    /// \u{fdab}: 'ﶫ'
    ArabicLigatureDadWithHahWithYehFinalForm,
    /// \u{fdac}: 'ﶬ'
    ArabicLigatureLamWithJeemWithYehFinalForm,
    /// \u{fdad}: 'ﶭ'
    ArabicLigatureLamWithMeemWithYehFinalForm,
    /// \u{fdae}: 'ﶮ'
    ArabicLigatureYehWithHahWithYehFinalForm,
    /// \u{fdaf}: 'ﶯ'
    ArabicLigatureYehWithJeemWithYehFinalForm,
    /// \u{fdb0}: 'ﶰ'
    ArabicLigatureYehWithMeemWithYehFinalForm,
    /// \u{fdb1}: 'ﶱ'
    ArabicLigatureMeemWithMeemWithYehFinalForm,
    /// \u{fdb2}: 'ﶲ'
    ArabicLigatureQafWithMeemWithYehFinalForm,
    /// \u{fdb3}: 'ﶳ'
    ArabicLigatureNoonWithHahWithYehFinalForm,
    /// \u{fdb4}: 'ﶴ'
    ArabicLigatureQafWithMeemWithHahInitialForm,
    /// \u{fdb5}: 'ﶵ'
    ArabicLigatureLamWithHahWithMeemInitialForm,
    /// \u{fdb6}: 'ﶶ'
    ArabicLigatureAinWithMeemWithYehFinalForm,
    /// \u{fdb7}: 'ﶷ'
    ArabicLigatureKafWithMeemWithYehFinalForm,
    /// \u{fdb8}: 'ﶸ'
    ArabicLigatureNoonWithJeemWithHahInitialForm,
    /// \u{fdb9}: 'ﶹ'
    ArabicLigatureMeemWithKhahWithYehFinalForm,
    /// \u{fdba}: 'ﶺ'
    ArabicLigatureLamWithJeemWithMeemInitialForm,
    /// \u{fdbb}: 'ﶻ'
    ArabicLigatureKafWithMeemWithMeemFinalForm,
    /// \u{fdbc}: 'ﶼ'
    ArabicLigatureLamWithJeemWithMeemFinalForm,
    /// \u{fdbd}: 'ﶽ'
    ArabicLigatureNoonWithJeemWithHahFinalForm,
    /// \u{fdbe}: 'ﶾ'
    ArabicLigatureJeemWithHahWithYehFinalForm,
    /// \u{fdbf}: 'ﶿ'
    ArabicLigatureHahWithJeemWithYehFinalForm,
    /// \u{fdc0}: 'ﷀ'
    ArabicLigatureMeemWithJeemWithYehFinalForm,
    /// \u{fdc1}: 'ﷁ'
    ArabicLigatureFehWithMeemWithYehFinalForm,
    /// \u{fdc2}: 'ﷂ'
    ArabicLigatureBehWithHahWithYehFinalForm,
    /// \u{fdc3}: 'ﷃ'
    ArabicLigatureKafWithMeemWithMeemInitialForm,
    /// \u{fdc4}: 'ﷄ'
    ArabicLigatureAinWithJeemWithMeemInitialForm,
    /// \u{fdc5}: 'ﷅ'
    ArabicLigatureSadWithMeemWithMeemInitialForm,
    /// \u{fdc6}: 'ﷆ'
    ArabicLigatureSeenWithKhahWithYehFinalForm,
    /// \u{fdc7}: 'ﷇ'
    ArabicLigatureNoonWithJeemWithYehFinalForm,
    /// \u{fdf0}: 'ﷰ'
    ArabicLigatureSallaUsedAsKoranicStopSignIsolatedForm,
    /// \u{fdf1}: 'ﷱ'
    ArabicLigatureQalaUsedAsKoranicStopSignIsolatedForm,
    /// \u{fdf2}: 'ﷲ'
    ArabicLigatureAllahIsolatedForm,
    /// \u{fdf3}: 'ﷳ'
    ArabicLigatureAkbarIsolatedForm,
    /// \u{fdf4}: 'ﷴ'
    ArabicLigatureMohammadIsolatedForm,
    /// \u{fdf5}: 'ﷵ'
    ArabicLigatureSalamIsolatedForm,
    /// \u{fdf6}: 'ﷶ'
    ArabicLigatureRasoulIsolatedForm,
    /// \u{fdf7}: 'ﷷ'
    ArabicLigatureAlayheIsolatedForm,
    /// \u{fdf8}: 'ﷸ'
    ArabicLigatureWasallamIsolatedForm,
    /// \u{fdf9}: 'ﷹ'
    ArabicLigatureSallaIsolatedForm,
    /// \u{fdfa}: 'ﷺ'
    ArabicLigatureSallallahouAlayheWasallam,
    /// \u{fdfb}: 'ﷻ'
    ArabicLigatureJallajalalouhou,
    /// \u{fdfc}: '﷼'
    RialSign,
    /// \u{fdfd}: '﷽'
    ArabicLigatureBismillahArDashRahmanArDashRaheem,
}

impl Into<char> for ArabicPresentationFormsA {
    fn into(self) -> char {
        match self {
            ArabicPresentationFormsA::ArabicLetterAlefWaslaIsolatedForm => 'ﭐ',
            ArabicPresentationFormsA::ArabicLetterAlefWaslaFinalForm => 'ﭑ',
            ArabicPresentationFormsA::ArabicLetterBeehIsolatedForm => 'ﭒ',
            ArabicPresentationFormsA::ArabicLetterBeehFinalForm => 'ﭓ',
            ArabicPresentationFormsA::ArabicLetterBeehInitialForm => 'ﭔ',
            ArabicPresentationFormsA::ArabicLetterBeehMedialForm => 'ﭕ',
            ArabicPresentationFormsA::ArabicLetterPehIsolatedForm => 'ﭖ',
            ArabicPresentationFormsA::ArabicLetterPehFinalForm => 'ﭗ',
            ArabicPresentationFormsA::ArabicLetterPehInitialForm => 'ﭘ',
            ArabicPresentationFormsA::ArabicLetterPehMedialForm => 'ﭙ',
            ArabicPresentationFormsA::ArabicLetterBehehIsolatedForm => 'ﭚ',
            ArabicPresentationFormsA::ArabicLetterBehehFinalForm => 'ﭛ',
            ArabicPresentationFormsA::ArabicLetterBehehInitialForm => 'ﭜ',
            ArabicPresentationFormsA::ArabicLetterBehehMedialForm => 'ﭝ',
            ArabicPresentationFormsA::ArabicLetterTtehehIsolatedForm => 'ﭞ',
            ArabicPresentationFormsA::ArabicLetterTtehehFinalForm => 'ﭟ',
            ArabicPresentationFormsA::ArabicLetterTtehehInitialForm => 'ﭠ',
            ArabicPresentationFormsA::ArabicLetterTtehehMedialForm => 'ﭡ',
            ArabicPresentationFormsA::ArabicLetterTehehIsolatedForm => 'ﭢ',
            ArabicPresentationFormsA::ArabicLetterTehehFinalForm => 'ﭣ',
            ArabicPresentationFormsA::ArabicLetterTehehInitialForm => 'ﭤ',
            ArabicPresentationFormsA::ArabicLetterTehehMedialForm => 'ﭥ',
            ArabicPresentationFormsA::ArabicLetterTtehIsolatedForm => 'ﭦ',
            ArabicPresentationFormsA::ArabicLetterTtehFinalForm => 'ﭧ',
            ArabicPresentationFormsA::ArabicLetterTtehInitialForm => 'ﭨ',
            ArabicPresentationFormsA::ArabicLetterTtehMedialForm => 'ﭩ',
            ArabicPresentationFormsA::ArabicLetterVehIsolatedForm => 'ﭪ',
            ArabicPresentationFormsA::ArabicLetterVehFinalForm => 'ﭫ',
            ArabicPresentationFormsA::ArabicLetterVehInitialForm => 'ﭬ',
            ArabicPresentationFormsA::ArabicLetterVehMedialForm => 'ﭭ',
            ArabicPresentationFormsA::ArabicLetterPehehIsolatedForm => 'ﭮ',
            ArabicPresentationFormsA::ArabicLetterPehehFinalForm => 'ﭯ',
            ArabicPresentationFormsA::ArabicLetterPehehInitialForm => 'ﭰ',
            ArabicPresentationFormsA::ArabicLetterPehehMedialForm => 'ﭱ',
            ArabicPresentationFormsA::ArabicLetterDyehIsolatedForm => 'ﭲ',
            ArabicPresentationFormsA::ArabicLetterDyehFinalForm => 'ﭳ',
            ArabicPresentationFormsA::ArabicLetterDyehInitialForm => 'ﭴ',
            ArabicPresentationFormsA::ArabicLetterDyehMedialForm => 'ﭵ',
            ArabicPresentationFormsA::ArabicLetterNyehIsolatedForm => 'ﭶ',
            ArabicPresentationFormsA::ArabicLetterNyehFinalForm => 'ﭷ',
            ArabicPresentationFormsA::ArabicLetterNyehInitialForm => 'ﭸ',
            ArabicPresentationFormsA::ArabicLetterNyehMedialForm => 'ﭹ',
            ArabicPresentationFormsA::ArabicLetterTchehIsolatedForm => 'ﭺ',
            ArabicPresentationFormsA::ArabicLetterTchehFinalForm => 'ﭻ',
            ArabicPresentationFormsA::ArabicLetterTchehInitialForm => 'ﭼ',
            ArabicPresentationFormsA::ArabicLetterTchehMedialForm => 'ﭽ',
            ArabicPresentationFormsA::ArabicLetterTchehehIsolatedForm => 'ﭾ',
            ArabicPresentationFormsA::ArabicLetterTchehehFinalForm => 'ﭿ',
            ArabicPresentationFormsA::ArabicLetterTchehehInitialForm => 'ﮀ',
            ArabicPresentationFormsA::ArabicLetterTchehehMedialForm => 'ﮁ',
            ArabicPresentationFormsA::ArabicLetterDdahalIsolatedForm => 'ﮂ',
            ArabicPresentationFormsA::ArabicLetterDdahalFinalForm => 'ﮃ',
            ArabicPresentationFormsA::ArabicLetterDahalIsolatedForm => 'ﮄ',
            ArabicPresentationFormsA::ArabicLetterDahalFinalForm => 'ﮅ',
            ArabicPresentationFormsA::ArabicLetterDulIsolatedForm => 'ﮆ',
            ArabicPresentationFormsA::ArabicLetterDulFinalForm => 'ﮇ',
            ArabicPresentationFormsA::ArabicLetterDdalIsolatedForm => 'ﮈ',
            ArabicPresentationFormsA::ArabicLetterDdalFinalForm => 'ﮉ',
            ArabicPresentationFormsA::ArabicLetterJehIsolatedForm => 'ﮊ',
            ArabicPresentationFormsA::ArabicLetterJehFinalForm => 'ﮋ',
            ArabicPresentationFormsA::ArabicLetterRrehIsolatedForm => 'ﮌ',
            ArabicPresentationFormsA::ArabicLetterRrehFinalForm => 'ﮍ',
            ArabicPresentationFormsA::ArabicLetterKehehIsolatedForm => 'ﮎ',
            ArabicPresentationFormsA::ArabicLetterKehehFinalForm => 'ﮏ',
            ArabicPresentationFormsA::ArabicLetterKehehInitialForm => 'ﮐ',
            ArabicPresentationFormsA::ArabicLetterKehehMedialForm => 'ﮑ',
            ArabicPresentationFormsA::ArabicLetterGafIsolatedForm => 'ﮒ',
            ArabicPresentationFormsA::ArabicLetterGafFinalForm => 'ﮓ',
            ArabicPresentationFormsA::ArabicLetterGafInitialForm => 'ﮔ',
            ArabicPresentationFormsA::ArabicLetterGafMedialForm => 'ﮕ',
            ArabicPresentationFormsA::ArabicLetterGuehIsolatedForm => 'ﮖ',
            ArabicPresentationFormsA::ArabicLetterGuehFinalForm => 'ﮗ',
            ArabicPresentationFormsA::ArabicLetterGuehInitialForm => 'ﮘ',
            ArabicPresentationFormsA::ArabicLetterGuehMedialForm => 'ﮙ',
            ArabicPresentationFormsA::ArabicLetterNgoehIsolatedForm => 'ﮚ',
            ArabicPresentationFormsA::ArabicLetterNgoehFinalForm => 'ﮛ',
            ArabicPresentationFormsA::ArabicLetterNgoehInitialForm => 'ﮜ',
            ArabicPresentationFormsA::ArabicLetterNgoehMedialForm => 'ﮝ',
            ArabicPresentationFormsA::ArabicLetterNoonGhunnaIsolatedForm => 'ﮞ',
            ArabicPresentationFormsA::ArabicLetterNoonGhunnaFinalForm => 'ﮟ',
            ArabicPresentationFormsA::ArabicLetterRnoonIsolatedForm => 'ﮠ',
            ArabicPresentationFormsA::ArabicLetterRnoonFinalForm => 'ﮡ',
            ArabicPresentationFormsA::ArabicLetterRnoonInitialForm => 'ﮢ',
            ArabicPresentationFormsA::ArabicLetterRnoonMedialForm => 'ﮣ',
            ArabicPresentationFormsA::ArabicLetterHehWithYehAboveIsolatedForm => 'ﮤ',
            ArabicPresentationFormsA::ArabicLetterHehWithYehAboveFinalForm => 'ﮥ',
            ArabicPresentationFormsA::ArabicLetterHehGoalIsolatedForm => 'ﮦ',
            ArabicPresentationFormsA::ArabicLetterHehGoalFinalForm => 'ﮧ',
            ArabicPresentationFormsA::ArabicLetterHehGoalInitialForm => 'ﮨ',
            ArabicPresentationFormsA::ArabicLetterHehGoalMedialForm => 'ﮩ',
            ArabicPresentationFormsA::ArabicLetterHehDoachashmeeIsolatedForm => 'ﮪ',
            ArabicPresentationFormsA::ArabicLetterHehDoachashmeeFinalForm => 'ﮫ',
            ArabicPresentationFormsA::ArabicLetterHehDoachashmeeInitialForm => 'ﮬ',
            ArabicPresentationFormsA::ArabicLetterHehDoachashmeeMedialForm => 'ﮭ',
            ArabicPresentationFormsA::ArabicLetterYehBarreeIsolatedForm => 'ﮮ',
            ArabicPresentationFormsA::ArabicLetterYehBarreeFinalForm => 'ﮯ',
            ArabicPresentationFormsA::ArabicLetterYehBarreeWithHamzaAboveIsolatedForm => 'ﮰ',
            ArabicPresentationFormsA::ArabicLetterYehBarreeWithHamzaAboveFinalForm => 'ﮱ',
            ArabicPresentationFormsA::ArabicSymbolDotAbove => '﮲',
            ArabicPresentationFormsA::ArabicSymbolDotBelow => '﮳',
            ArabicPresentationFormsA::ArabicSymbolTwoDotsAbove => '﮴',
            ArabicPresentationFormsA::ArabicSymbolTwoDotsBelow => '﮵',
            ArabicPresentationFormsA::ArabicSymbolThreeDotsAbove => '﮶',
            ArabicPresentationFormsA::ArabicSymbolThreeDotsBelow => '﮷',
            ArabicPresentationFormsA::ArabicSymbolThreeDotsPointingDownwardsAbove => '﮸',
            ArabicPresentationFormsA::ArabicSymbolThreeDotsPointingDownwardsBelow => '﮹',
            ArabicPresentationFormsA::ArabicSymbolFourDotsAbove => '﮺',
            ArabicPresentationFormsA::ArabicSymbolFourDotsBelow => '﮻',
            ArabicPresentationFormsA::ArabicSymbolDoubleVerticalBarBelow => '﮼',
            ArabicPresentationFormsA::ArabicSymbolTwoDotsVerticallyAbove => '﮽',
            ArabicPresentationFormsA::ArabicSymbolTwoDotsVerticallyBelow => '﮾',
            ArabicPresentationFormsA::ArabicSymbolRing => '﮿',
            ArabicPresentationFormsA::ArabicSymbolSmallTahAbove => '﯀',
            ArabicPresentationFormsA::ArabicSymbolSmallTahBelow => '﯁',
            ArabicPresentationFormsA::ArabicLetterNgIsolatedForm => 'ﯓ',
            ArabicPresentationFormsA::ArabicLetterNgFinalForm => 'ﯔ',
            ArabicPresentationFormsA::ArabicLetterNgInitialForm => 'ﯕ',
            ArabicPresentationFormsA::ArabicLetterNgMedialForm => 'ﯖ',
            ArabicPresentationFormsA::ArabicLetterUIsolatedForm => 'ﯗ',
            ArabicPresentationFormsA::ArabicLetterUFinalForm => 'ﯘ',
            ArabicPresentationFormsA::ArabicLetterOeIsolatedForm => 'ﯙ',
            ArabicPresentationFormsA::ArabicLetterOeFinalForm => 'ﯚ',
            ArabicPresentationFormsA::ArabicLetterYuIsolatedForm => 'ﯛ',
            ArabicPresentationFormsA::ArabicLetterYuFinalForm => 'ﯜ',
            ArabicPresentationFormsA::ArabicLetterUWithHamzaAboveIsolatedForm => 'ﯝ',
            ArabicPresentationFormsA::ArabicLetterVeIsolatedForm => 'ﯞ',
            ArabicPresentationFormsA::ArabicLetterVeFinalForm => 'ﯟ',
            ArabicPresentationFormsA::ArabicLetterKirghizOeIsolatedForm => 'ﯠ',
            ArabicPresentationFormsA::ArabicLetterKirghizOeFinalForm => 'ﯡ',
            ArabicPresentationFormsA::ArabicLetterKirghizYuIsolatedForm => 'ﯢ',
            ArabicPresentationFormsA::ArabicLetterKirghizYuFinalForm => 'ﯣ',
            ArabicPresentationFormsA::ArabicLetterEIsolatedForm => 'ﯤ',
            ArabicPresentationFormsA::ArabicLetterEFinalForm => 'ﯥ',
            ArabicPresentationFormsA::ArabicLetterEInitialForm => 'ﯦ',
            ArabicPresentationFormsA::ArabicLetterEMedialForm => 'ﯧ',
            ArabicPresentationFormsA::ArabicLetterUighurKazakhKirghizAlefMaksuraInitialForm => 'ﯨ',
            ArabicPresentationFormsA::ArabicLetterUighurKazakhKirghizAlefMaksuraMedialForm => 'ﯩ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefIsolatedForm => 'ﯪ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefFinalForm => 'ﯫ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAeIsolatedForm => 'ﯬ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAeFinalForm => 'ﯭ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithWawIsolatedForm => 'ﯮ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithWawFinalForm => 'ﯯ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithUIsolatedForm => 'ﯰ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithUFinalForm => 'ﯱ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithOeIsolatedForm => 'ﯲ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithOeFinalForm => 'ﯳ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYuIsolatedForm => 'ﯴ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYuFinalForm => 'ﯵ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEIsolatedForm => 'ﯶ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEFinalForm => 'ﯷ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEInitialForm => 'ﯸ',
            ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraIsolatedForm => 'ﯹ',
            ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraFinalForm => 'ﯺ',
            ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraInitialForm => 'ﯻ',
            ArabicPresentationFormsA::ArabicLetterFarsiYehIsolatedForm => 'ﯼ',
            ArabicPresentationFormsA::ArabicLetterFarsiYehFinalForm => 'ﯽ',
            ArabicPresentationFormsA::ArabicLetterFarsiYehInitialForm => 'ﯾ',
            ArabicPresentationFormsA::ArabicLetterFarsiYehMedialForm => 'ﯿ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithJeemIsolatedForm => 'ﰀ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHahIsolatedForm => 'ﰁ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemIsolatedForm => 'ﰂ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefMaksuraIsolatedForm => 'ﰃ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYehIsolatedForm => 'ﰄ',
            ArabicPresentationFormsA::ArabicLigatureBehWithJeemIsolatedForm => 'ﰅ',
            ArabicPresentationFormsA::ArabicLigatureBehWithHahIsolatedForm => 'ﰆ',
            ArabicPresentationFormsA::ArabicLigatureBehWithKhahIsolatedForm => 'ﰇ',
            ArabicPresentationFormsA::ArabicLigatureBehWithMeemIsolatedForm => 'ﰈ',
            ArabicPresentationFormsA::ArabicLigatureBehWithAlefMaksuraIsolatedForm => 'ﰉ',
            ArabicPresentationFormsA::ArabicLigatureBehWithYehIsolatedForm => 'ﰊ',
            ArabicPresentationFormsA::ArabicLigatureTehWithJeemIsolatedForm => 'ﰋ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHahIsolatedForm => 'ﰌ',
            ArabicPresentationFormsA::ArabicLigatureTehWithKhahIsolatedForm => 'ﰍ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemIsolatedForm => 'ﰎ',
            ArabicPresentationFormsA::ArabicLigatureTehWithAlefMaksuraIsolatedForm => 'ﰏ',
            ArabicPresentationFormsA::ArabicLigatureTehWithYehIsolatedForm => 'ﰐ',
            ArabicPresentationFormsA::ArabicLigatureThehWithJeemIsolatedForm => 'ﰑ',
            ArabicPresentationFormsA::ArabicLigatureThehWithMeemIsolatedForm => 'ﰒ',
            ArabicPresentationFormsA::ArabicLigatureThehWithAlefMaksuraIsolatedForm => 'ﰓ',
            ArabicPresentationFormsA::ArabicLigatureThehWithYehIsolatedForm => 'ﰔ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithHahIsolatedForm => 'ﰕ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemIsolatedForm => 'ﰖ',
            ArabicPresentationFormsA::ArabicLigatureHahWithJeemIsolatedForm => 'ﰗ',
            ArabicPresentationFormsA::ArabicLigatureHahWithMeemIsolatedForm => 'ﰘ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithJeemIsolatedForm => 'ﰙ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithHahIsolatedForm => 'ﰚ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithMeemIsolatedForm => 'ﰛ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithJeemIsolatedForm => 'ﰜ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHahIsolatedForm => 'ﰝ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithKhahIsolatedForm => 'ﰞ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemIsolatedForm => 'ﰟ',
            ArabicPresentationFormsA::ArabicLigatureSadWithHahIsolatedForm => 'ﰠ',
            ArabicPresentationFormsA::ArabicLigatureSadWithMeemIsolatedForm => 'ﰡ',
            ArabicPresentationFormsA::ArabicLigatureDadWithJeemIsolatedForm => 'ﰢ',
            ArabicPresentationFormsA::ArabicLigatureDadWithHahIsolatedForm => 'ﰣ',
            ArabicPresentationFormsA::ArabicLigatureDadWithKhahIsolatedForm => 'ﰤ',
            ArabicPresentationFormsA::ArabicLigatureDadWithMeemIsolatedForm => 'ﰥ',
            ArabicPresentationFormsA::ArabicLigatureTahWithHahIsolatedForm => 'ﰦ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemIsolatedForm => 'ﰧ',
            ArabicPresentationFormsA::ArabicLigatureZahWithMeemIsolatedForm => 'ﰨ',
            ArabicPresentationFormsA::ArabicLigatureAinWithJeemIsolatedForm => 'ﰩ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemIsolatedForm => 'ﰪ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithJeemIsolatedForm => 'ﰫ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithMeemIsolatedForm => 'ﰬ',
            ArabicPresentationFormsA::ArabicLigatureFehWithJeemIsolatedForm => 'ﰭ',
            ArabicPresentationFormsA::ArabicLigatureFehWithHahIsolatedForm => 'ﰮ',
            ArabicPresentationFormsA::ArabicLigatureFehWithKhahIsolatedForm => 'ﰯ',
            ArabicPresentationFormsA::ArabicLigatureFehWithMeemIsolatedForm => 'ﰰ',
            ArabicPresentationFormsA::ArabicLigatureFehWithAlefMaksuraIsolatedForm => 'ﰱ',
            ArabicPresentationFormsA::ArabicLigatureFehWithYehIsolatedForm => 'ﰲ',
            ArabicPresentationFormsA::ArabicLigatureQafWithHahIsolatedForm => 'ﰳ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemIsolatedForm => 'ﰴ',
            ArabicPresentationFormsA::ArabicLigatureQafWithAlefMaksuraIsolatedForm => 'ﰵ',
            ArabicPresentationFormsA::ArabicLigatureQafWithYehIsolatedForm => 'ﰶ',
            ArabicPresentationFormsA::ArabicLigatureKafWithAlefIsolatedForm => 'ﰷ',
            ArabicPresentationFormsA::ArabicLigatureKafWithJeemIsolatedForm => 'ﰸ',
            ArabicPresentationFormsA::ArabicLigatureKafWithHahIsolatedForm => 'ﰹ',
            ArabicPresentationFormsA::ArabicLigatureKafWithKhahIsolatedForm => 'ﰺ',
            ArabicPresentationFormsA::ArabicLigatureKafWithLamIsolatedForm => 'ﰻ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemIsolatedForm => 'ﰼ',
            ArabicPresentationFormsA::ArabicLigatureKafWithAlefMaksuraIsolatedForm => 'ﰽ',
            ArabicPresentationFormsA::ArabicLigatureKafWithYehIsolatedForm => 'ﰾ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemIsolatedForm => 'ﰿ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahIsolatedForm => 'ﱀ',
            ArabicPresentationFormsA::ArabicLigatureLamWithKhahIsolatedForm => 'ﱁ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemIsolatedForm => 'ﱂ',
            ArabicPresentationFormsA::ArabicLigatureLamWithAlefMaksuraIsolatedForm => 'ﱃ',
            ArabicPresentationFormsA::ArabicLigatureLamWithYehIsolatedForm => 'ﱄ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemIsolatedForm => 'ﱅ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithHahIsolatedForm => 'ﱆ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithKhahIsolatedForm => 'ﱇ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithMeemIsolatedForm => 'ﱈ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithAlefMaksuraIsolatedForm => 'ﱉ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithYehIsolatedForm => 'ﱊ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemIsolatedForm => 'ﱋ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHahIsolatedForm => 'ﱌ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithKhahIsolatedForm => 'ﱍ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemIsolatedForm => 'ﱎ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithAlefMaksuraIsolatedForm => 'ﱏ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithYehIsolatedForm => 'ﱐ',
            ArabicPresentationFormsA::ArabicLigatureHehWithJeemIsolatedForm => 'ﱑ',
            ArabicPresentationFormsA::ArabicLigatureHehWithMeemIsolatedForm => 'ﱒ',
            ArabicPresentationFormsA::ArabicLigatureHehWithAlefMaksuraIsolatedForm => 'ﱓ',
            ArabicPresentationFormsA::ArabicLigatureHehWithYehIsolatedForm => 'ﱔ',
            ArabicPresentationFormsA::ArabicLigatureYehWithJeemIsolatedForm => 'ﱕ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHahIsolatedForm => 'ﱖ',
            ArabicPresentationFormsA::ArabicLigatureYehWithKhahIsolatedForm => 'ﱗ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemIsolatedForm => 'ﱘ',
            ArabicPresentationFormsA::ArabicLigatureYehWithAlefMaksuraIsolatedForm => 'ﱙ',
            ArabicPresentationFormsA::ArabicLigatureYehWithYehIsolatedForm => 'ﱚ',
            ArabicPresentationFormsA::ArabicLigatureThalWithSuperscriptAlefIsolatedForm => 'ﱛ',
            ArabicPresentationFormsA::ArabicLigatureRehWithSuperscriptAlefIsolatedForm => 'ﱜ',
            ArabicPresentationFormsA::ArabicLigatureAlefMaksuraWithSuperscriptAlefIsolatedForm => 'ﱝ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithDammatanIsolatedForm => 'ﱞ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithKasratanIsolatedForm => 'ﱟ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithFathaIsolatedForm => 'ﱠ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithDammaIsolatedForm => 'ﱡ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithKasraIsolatedForm => 'ﱢ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithSuperscriptAlefIsolatedForm => 'ﱣ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithRehFinalForm => 'ﱤ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithZainFinalForm => 'ﱥ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemFinalForm => 'ﱦ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithNoonFinalForm => 'ﱧ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefMaksuraFinalForm => 'ﱨ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYehFinalForm => 'ﱩ',
            ArabicPresentationFormsA::ArabicLigatureBehWithRehFinalForm => 'ﱪ',
            ArabicPresentationFormsA::ArabicLigatureBehWithZainFinalForm => 'ﱫ',
            ArabicPresentationFormsA::ArabicLigatureBehWithMeemFinalForm => 'ﱬ',
            ArabicPresentationFormsA::ArabicLigatureBehWithNoonFinalForm => 'ﱭ',
            ArabicPresentationFormsA::ArabicLigatureBehWithAlefMaksuraFinalForm => 'ﱮ',
            ArabicPresentationFormsA::ArabicLigatureBehWithYehFinalForm => 'ﱯ',
            ArabicPresentationFormsA::ArabicLigatureTehWithRehFinalForm => 'ﱰ',
            ArabicPresentationFormsA::ArabicLigatureTehWithZainFinalForm => 'ﱱ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemFinalForm => 'ﱲ',
            ArabicPresentationFormsA::ArabicLigatureTehWithNoonFinalForm => 'ﱳ',
            ArabicPresentationFormsA::ArabicLigatureTehWithAlefMaksuraFinalForm => 'ﱴ',
            ArabicPresentationFormsA::ArabicLigatureTehWithYehFinalForm => 'ﱵ',
            ArabicPresentationFormsA::ArabicLigatureThehWithRehFinalForm => 'ﱶ',
            ArabicPresentationFormsA::ArabicLigatureThehWithZainFinalForm => 'ﱷ',
            ArabicPresentationFormsA::ArabicLigatureThehWithMeemFinalForm => 'ﱸ',
            ArabicPresentationFormsA::ArabicLigatureThehWithNoonFinalForm => 'ﱹ',
            ArabicPresentationFormsA::ArabicLigatureThehWithAlefMaksuraFinalForm => 'ﱺ',
            ArabicPresentationFormsA::ArabicLigatureThehWithYehFinalForm => 'ﱻ',
            ArabicPresentationFormsA::ArabicLigatureFehWithAlefMaksuraFinalForm => 'ﱼ',
            ArabicPresentationFormsA::ArabicLigatureFehWithYehFinalForm => 'ﱽ',
            ArabicPresentationFormsA::ArabicLigatureQafWithAlefMaksuraFinalForm => 'ﱾ',
            ArabicPresentationFormsA::ArabicLigatureQafWithYehFinalForm => 'ﱿ',
            ArabicPresentationFormsA::ArabicLigatureKafWithAlefFinalForm => 'ﲀ',
            ArabicPresentationFormsA::ArabicLigatureKafWithLamFinalForm => 'ﲁ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemFinalForm => 'ﲂ',
            ArabicPresentationFormsA::ArabicLigatureKafWithAlefMaksuraFinalForm => 'ﲃ',
            ArabicPresentationFormsA::ArabicLigatureKafWithYehFinalForm => 'ﲄ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemFinalForm => 'ﲅ',
            ArabicPresentationFormsA::ArabicLigatureLamWithAlefMaksuraFinalForm => 'ﲆ',
            ArabicPresentationFormsA::ArabicLigatureLamWithYehFinalForm => 'ﲇ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithAlefFinalForm => 'ﲈ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithMeemFinalForm => 'ﲉ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithRehFinalForm => 'ﲊ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithZainFinalForm => 'ﲋ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemFinalForm => 'ﲌ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithNoonFinalForm => 'ﲍ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithAlefMaksuraFinalForm => 'ﲎ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithYehFinalForm => 'ﲏ',
            ArabicPresentationFormsA::ArabicLigatureAlefMaksuraWithSuperscriptAlefFinalForm => 'ﲐ',
            ArabicPresentationFormsA::ArabicLigatureYehWithRehFinalForm => 'ﲑ',
            ArabicPresentationFormsA::ArabicLigatureYehWithZainFinalForm => 'ﲒ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemFinalForm => 'ﲓ',
            ArabicPresentationFormsA::ArabicLigatureYehWithNoonFinalForm => 'ﲔ',
            ArabicPresentationFormsA::ArabicLigatureYehWithAlefMaksuraFinalForm => 'ﲕ',
            ArabicPresentationFormsA::ArabicLigatureYehWithYehFinalForm => 'ﲖ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithJeemInitialForm => 'ﲗ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHahInitialForm => 'ﲘ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithKhahInitialForm => 'ﲙ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemInitialForm => 'ﲚ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHehInitialForm => 'ﲛ',
            ArabicPresentationFormsA::ArabicLigatureBehWithJeemInitialForm => 'ﲜ',
            ArabicPresentationFormsA::ArabicLigatureBehWithHahInitialForm => 'ﲝ',
            ArabicPresentationFormsA::ArabicLigatureBehWithKhahInitialForm => 'ﲞ',
            ArabicPresentationFormsA::ArabicLigatureBehWithMeemInitialForm => 'ﲟ',
            ArabicPresentationFormsA::ArabicLigatureBehWithHehInitialForm => 'ﲠ',
            ArabicPresentationFormsA::ArabicLigatureTehWithJeemInitialForm => 'ﲡ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHahInitialForm => 'ﲢ',
            ArabicPresentationFormsA::ArabicLigatureTehWithKhahInitialForm => 'ﲣ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemInitialForm => 'ﲤ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHehInitialForm => 'ﲥ',
            ArabicPresentationFormsA::ArabicLigatureThehWithMeemInitialForm => 'ﲦ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithHahInitialForm => 'ﲧ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemInitialForm => 'ﲨ',
            ArabicPresentationFormsA::ArabicLigatureHahWithJeemInitialForm => 'ﲩ',
            ArabicPresentationFormsA::ArabicLigatureHahWithMeemInitialForm => 'ﲪ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithJeemInitialForm => 'ﲫ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithMeemInitialForm => 'ﲬ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithJeemInitialForm => 'ﲭ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHahInitialForm => 'ﲮ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithKhahInitialForm => 'ﲯ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemInitialForm => 'ﲰ',
            ArabicPresentationFormsA::ArabicLigatureSadWithHahInitialForm => 'ﲱ',
            ArabicPresentationFormsA::ArabicLigatureSadWithKhahInitialForm => 'ﲲ',
            ArabicPresentationFormsA::ArabicLigatureSadWithMeemInitialForm => 'ﲳ',
            ArabicPresentationFormsA::ArabicLigatureDadWithJeemInitialForm => 'ﲴ',
            ArabicPresentationFormsA::ArabicLigatureDadWithHahInitialForm => 'ﲵ',
            ArabicPresentationFormsA::ArabicLigatureDadWithKhahInitialForm => 'ﲶ',
            ArabicPresentationFormsA::ArabicLigatureDadWithMeemInitialForm => 'ﲷ',
            ArabicPresentationFormsA::ArabicLigatureTahWithHahInitialForm => 'ﲸ',
            ArabicPresentationFormsA::ArabicLigatureZahWithMeemInitialForm => 'ﲹ',
            ArabicPresentationFormsA::ArabicLigatureAinWithJeemInitialForm => 'ﲺ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemInitialForm => 'ﲻ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithJeemInitialForm => 'ﲼ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithMeemInitialForm => 'ﲽ',
            ArabicPresentationFormsA::ArabicLigatureFehWithJeemInitialForm => 'ﲾ',
            ArabicPresentationFormsA::ArabicLigatureFehWithHahInitialForm => 'ﲿ',
            ArabicPresentationFormsA::ArabicLigatureFehWithKhahInitialForm => 'ﳀ',
            ArabicPresentationFormsA::ArabicLigatureFehWithMeemInitialForm => 'ﳁ',
            ArabicPresentationFormsA::ArabicLigatureQafWithHahInitialForm => 'ﳂ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemInitialForm => 'ﳃ',
            ArabicPresentationFormsA::ArabicLigatureKafWithJeemInitialForm => 'ﳄ',
            ArabicPresentationFormsA::ArabicLigatureKafWithHahInitialForm => 'ﳅ',
            ArabicPresentationFormsA::ArabicLigatureKafWithKhahInitialForm => 'ﳆ',
            ArabicPresentationFormsA::ArabicLigatureKafWithLamInitialForm => 'ﳇ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemInitialForm => 'ﳈ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemInitialForm => 'ﳉ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahInitialForm => 'ﳊ',
            ArabicPresentationFormsA::ArabicLigatureLamWithKhahInitialForm => 'ﳋ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemInitialForm => 'ﳌ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHehInitialForm => 'ﳍ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemInitialForm => 'ﳎ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithHahInitialForm => 'ﳏ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithKhahInitialForm => 'ﳐ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithMeemInitialForm => 'ﳑ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemInitialForm => 'ﳒ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHahInitialForm => 'ﳓ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithKhahInitialForm => 'ﳔ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemInitialForm => 'ﳕ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHehInitialForm => 'ﳖ',
            ArabicPresentationFormsA::ArabicLigatureHehWithJeemInitialForm => 'ﳗ',
            ArabicPresentationFormsA::ArabicLigatureHehWithMeemInitialForm => 'ﳘ',
            ArabicPresentationFormsA::ArabicLigatureHehWithSuperscriptAlefInitialForm => 'ﳙ',
            ArabicPresentationFormsA::ArabicLigatureYehWithJeemInitialForm => 'ﳚ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHahInitialForm => 'ﳛ',
            ArabicPresentationFormsA::ArabicLigatureYehWithKhahInitialForm => 'ﳜ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemInitialForm => 'ﳝ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHehInitialForm => 'ﳞ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemMedialForm => 'ﳟ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHehMedialForm => 'ﳠ',
            ArabicPresentationFormsA::ArabicLigatureBehWithMeemMedialForm => 'ﳡ',
            ArabicPresentationFormsA::ArabicLigatureBehWithHehMedialForm => 'ﳢ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemMedialForm => 'ﳣ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHehMedialForm => 'ﳤ',
            ArabicPresentationFormsA::ArabicLigatureThehWithMeemMedialForm => 'ﳥ',
            ArabicPresentationFormsA::ArabicLigatureThehWithHehMedialForm => 'ﳦ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemMedialForm => 'ﳧ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHehMedialForm => 'ﳨ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemMedialForm => 'ﳩ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHehMedialForm => 'ﳪ',
            ArabicPresentationFormsA::ArabicLigatureKafWithLamMedialForm => 'ﳫ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemMedialForm => 'ﳬ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemMedialForm => 'ﳭ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemMedialForm => 'ﳮ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHehMedialForm => 'ﳯ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemMedialForm => 'ﳰ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHehMedialForm => 'ﳱ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithFathaMedialForm => 'ﳲ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithDammaMedialForm => 'ﳳ',
            ArabicPresentationFormsA::ArabicLigatureShaddaWithKasraMedialForm => 'ﳴ',
            ArabicPresentationFormsA::ArabicLigatureTahWithAlefMaksuraIsolatedForm => 'ﳵ',
            ArabicPresentationFormsA::ArabicLigatureTahWithYehIsolatedForm => 'ﳶ',
            ArabicPresentationFormsA::ArabicLigatureAinWithAlefMaksuraIsolatedForm => 'ﳷ',
            ArabicPresentationFormsA::ArabicLigatureAinWithYehIsolatedForm => 'ﳸ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithAlefMaksuraIsolatedForm => 'ﳹ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithYehIsolatedForm => 'ﳺ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithAlefMaksuraIsolatedForm => 'ﳻ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithYehIsolatedForm => 'ﳼ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithAlefMaksuraIsolatedForm => 'ﳽ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithYehIsolatedForm => 'ﳾ',
            ArabicPresentationFormsA::ArabicLigatureHahWithAlefMaksuraIsolatedForm => 'ﳿ',
            ArabicPresentationFormsA::ArabicLigatureHahWithYehIsolatedForm => 'ﴀ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithAlefMaksuraIsolatedForm => 'ﴁ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithYehIsolatedForm => 'ﴂ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithAlefMaksuraIsolatedForm => 'ﴃ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithYehIsolatedForm => 'ﴄ',
            ArabicPresentationFormsA::ArabicLigatureSadWithAlefMaksuraIsolatedForm => 'ﴅ',
            ArabicPresentationFormsA::ArabicLigatureSadWithYehIsolatedForm => 'ﴆ',
            ArabicPresentationFormsA::ArabicLigatureDadWithAlefMaksuraIsolatedForm => 'ﴇ',
            ArabicPresentationFormsA::ArabicLigatureDadWithYehIsolatedForm => 'ﴈ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithJeemIsolatedForm => 'ﴉ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahIsolatedForm => 'ﴊ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithKhahIsolatedForm => 'ﴋ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemIsolatedForm => 'ﴌ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithRehIsolatedForm => 'ﴍ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithRehIsolatedForm => 'ﴎ',
            ArabicPresentationFormsA::ArabicLigatureSadWithRehIsolatedForm => 'ﴏ',
            ArabicPresentationFormsA::ArabicLigatureDadWithRehIsolatedForm => 'ﴐ',
            ArabicPresentationFormsA::ArabicLigatureTahWithAlefMaksuraFinalForm => 'ﴑ',
            ArabicPresentationFormsA::ArabicLigatureTahWithYehFinalForm => 'ﴒ',
            ArabicPresentationFormsA::ArabicLigatureAinWithAlefMaksuraFinalForm => 'ﴓ',
            ArabicPresentationFormsA::ArabicLigatureAinWithYehFinalForm => 'ﴔ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithAlefMaksuraFinalForm => 'ﴕ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithYehFinalForm => 'ﴖ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithAlefMaksuraFinalForm => 'ﴗ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithYehFinalForm => 'ﴘ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithAlefMaksuraFinalForm => 'ﴙ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithYehFinalForm => 'ﴚ',
            ArabicPresentationFormsA::ArabicLigatureHahWithAlefMaksuraFinalForm => 'ﴛ',
            ArabicPresentationFormsA::ArabicLigatureHahWithYehFinalForm => 'ﴜ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithAlefMaksuraFinalForm => 'ﴝ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithYehFinalForm => 'ﴞ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithAlefMaksuraFinalForm => 'ﴟ',
            ArabicPresentationFormsA::ArabicLigatureKhahWithYehFinalForm => 'ﴠ',
            ArabicPresentationFormsA::ArabicLigatureSadWithAlefMaksuraFinalForm => 'ﴡ',
            ArabicPresentationFormsA::ArabicLigatureSadWithYehFinalForm => 'ﴢ',
            ArabicPresentationFormsA::ArabicLigatureDadWithAlefMaksuraFinalForm => 'ﴣ',
            ArabicPresentationFormsA::ArabicLigatureDadWithYehFinalForm => 'ﴤ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithJeemFinalForm => 'ﴥ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahFinalForm => 'ﴦ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithKhahFinalForm => 'ﴧ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemFinalForm => 'ﴨ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithRehFinalForm => 'ﴩ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithRehFinalForm => 'ﴪ',
            ArabicPresentationFormsA::ArabicLigatureSadWithRehFinalForm => 'ﴫ',
            ArabicPresentationFormsA::ArabicLigatureDadWithRehFinalForm => 'ﴬ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithJeemInitialForm => 'ﴭ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahInitialForm => 'ﴮ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithKhahInitialForm => 'ﴯ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemInitialForm => 'ﴰ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHehInitialForm => 'ﴱ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHehInitialForm => 'ﴲ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemInitialForm => 'ﴳ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithJeemMedialForm => 'ﴴ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHahMedialForm => 'ﴵ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithKhahMedialForm => 'ﴶ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithJeemMedialForm => 'ﴷ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahMedialForm => 'ﴸ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithKhahMedialForm => 'ﴹ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemMedialForm => 'ﴺ',
            ArabicPresentationFormsA::ArabicLigatureZahWithMeemMedialForm => 'ﴻ',
            ArabicPresentationFormsA::ArabicLigatureAlefWithFathatanFinalForm => 'ﴼ',
            ArabicPresentationFormsA::ArabicLigatureAlefWithFathatanIsolatedForm => 'ﴽ',
            ArabicPresentationFormsA::OrnateLeftParenthesis => '﴾',
            ArabicPresentationFormsA::OrnateRightParenthesis => '﴿',
            ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithMeemInitialForm => 'ﵐ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHahWithJeemFinalForm => 'ﵑ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHahWithJeemInitialForm => 'ﵒ',
            ArabicPresentationFormsA::ArabicLigatureTehWithHahWithMeemInitialForm => 'ﵓ',
            ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithMeemInitialForm => 'ﵔ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithJeemInitialForm => 'ﵕ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithHahInitialForm => 'ﵖ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithKhahInitialForm => 'ﵗ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithHahFinalForm => 'ﵘ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithHahInitialForm => 'ﵙ',
            ArabicPresentationFormsA::ArabicLigatureHahWithMeemWithYehFinalForm => 'ﵚ',
            ArabicPresentationFormsA::ArabicLigatureHahWithMeemWithAlefMaksuraFinalForm => 'ﵛ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithHahWithJeemInitialForm => 'ﵜ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithJeemWithHahInitialForm => 'ﵝ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithJeemWithAlefMaksuraFinalForm => 'ﵞ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithHahFinalForm => 'ﵟ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithHahInitialForm => 'ﵠ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithJeemInitialForm => 'ﵡ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithMeemFinalForm => 'ﵢ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithMeemInitialForm => 'ﵣ',
            ArabicPresentationFormsA::ArabicLigatureSadWithHahWithHahFinalForm => 'ﵤ',
            ArabicPresentationFormsA::ArabicLigatureSadWithHahWithHahInitialForm => 'ﵥ',
            ArabicPresentationFormsA::ArabicLigatureSadWithMeemWithMeemFinalForm => 'ﵦ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithMeemFinalForm => 'ﵧ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithMeemInitialForm => 'ﵨ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithJeemWithYehFinalForm => 'ﵩ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithKhahFinalForm => 'ﵪ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithKhahInitialForm => 'ﵫ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithMeemFinalForm => 'ﵬ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithMeemInitialForm => 'ﵭ',
            ArabicPresentationFormsA::ArabicLigatureDadWithHahWithAlefMaksuraFinalForm => 'ﵮ',
            ArabicPresentationFormsA::ArabicLigatureDadWithKhahWithMeemFinalForm => 'ﵯ',
            ArabicPresentationFormsA::ArabicLigatureDadWithKhahWithMeemInitialForm => 'ﵰ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithHahFinalForm => 'ﵱ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithHahInitialForm => 'ﵲ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithMeemInitialForm => 'ﵳ',
            ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithYehFinalForm => 'ﵴ',
            ArabicPresentationFormsA::ArabicLigatureAinWithJeemWithMeemFinalForm => 'ﵵ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithMeemFinalForm => 'ﵶ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithMeemInitialForm => 'ﵷ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithAlefMaksuraFinalForm => 'ﵸ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithMeemFinalForm => 'ﵹ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithYehFinalForm => 'ﵺ',
            ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithAlefMaksuraFinalForm => 'ﵻ',
            ArabicPresentationFormsA::ArabicLigatureFehWithKhahWithMeemFinalForm => 'ﵼ',
            ArabicPresentationFormsA::ArabicLigatureFehWithKhahWithMeemInitialForm => 'ﵽ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithHahFinalForm => 'ﵾ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithMeemFinalForm => 'ﵿ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahWithMeemFinalForm => 'ﶀ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahWithYehFinalForm => 'ﶁ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahWithAlefMaksuraFinalForm => 'ﶂ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithJeemInitialForm => 'ﶃ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithJeemFinalForm => 'ﶄ',
            ArabicPresentationFormsA::ArabicLigatureLamWithKhahWithMeemFinalForm => 'ﶅ',
            ArabicPresentationFormsA::ArabicLigatureLamWithKhahWithMeemInitialForm => 'ﶆ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithHahFinalForm => 'ﶇ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithHahInitialForm => 'ﶈ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithJeemInitialForm => 'ﶉ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithMeemInitialForm => 'ﶊ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithYehFinalForm => 'ﶋ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithHahInitialForm => 'ﶌ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithMeemInitialForm => 'ﶍ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithJeemInitialForm => 'ﶎ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithMeemInitialForm => 'ﶏ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithKhahInitialForm => 'ﶒ',
            ArabicPresentationFormsA::ArabicLigatureHehWithMeemWithJeemInitialForm => 'ﶓ',
            ArabicPresentationFormsA::ArabicLigatureHehWithMeemWithMeemInitialForm => 'ﶔ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithMeemInitialForm => 'ﶕ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithAlefMaksuraFinalForm => 'ﶖ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithMeemFinalForm => 'ﶗ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithMeemInitialForm => 'ﶘ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithAlefMaksuraFinalForm => 'ﶙ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemWithYehFinalForm => 'ﶚ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithMeemWithAlefMaksuraFinalForm => 'ﶛ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithMeemFinalForm => 'ﶜ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithMeemInitialForm => 'ﶝ',
            ArabicPresentationFormsA::ArabicLigatureBehWithKhahWithYehFinalForm => 'ﶞ',
            ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithYehFinalForm => 'ﶟ',
            ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithAlefMaksuraFinalForm => 'ﶠ',
            ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithYehFinalForm => 'ﶡ',
            ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithAlefMaksuraFinalForm => 'ﶢ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithYehFinalForm => 'ﶣ',
            ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithAlefMaksuraFinalForm => 'ﶤ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithYehFinalForm => 'ﶥ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithHahWithAlefMaksuraFinalForm => 'ﶦ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithAlefMaksuraFinalForm => 'ﶧ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithKhahWithAlefMaksuraFinalForm => 'ﶨ',
            ArabicPresentationFormsA::ArabicLigatureSadWithHahWithYehFinalForm => 'ﶩ',
            ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithYehFinalForm => 'ﶪ',
            ArabicPresentationFormsA::ArabicLigatureDadWithHahWithYehFinalForm => 'ﶫ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithYehFinalForm => 'ﶬ',
            ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithYehFinalForm => 'ﶭ',
            ArabicPresentationFormsA::ArabicLigatureYehWithHahWithYehFinalForm => 'ﶮ',
            ArabicPresentationFormsA::ArabicLigatureYehWithJeemWithYehFinalForm => 'ﶯ',
            ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithYehFinalForm => 'ﶰ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithMeemWithYehFinalForm => 'ﶱ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithYehFinalForm => 'ﶲ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithYehFinalForm => 'ﶳ',
            ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithHahInitialForm => 'ﶴ',
            ArabicPresentationFormsA::ArabicLigatureLamWithHahWithMeemInitialForm => 'ﶵ',
            ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithYehFinalForm => 'ﶶ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithYehFinalForm => 'ﶷ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithHahInitialForm => 'ﶸ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithYehFinalForm => 'ﶹ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithMeemInitialForm => 'ﶺ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithMeemFinalForm => 'ﶻ',
            ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithMeemFinalForm => 'ﶼ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithHahFinalForm => 'ﶽ',
            ArabicPresentationFormsA::ArabicLigatureJeemWithHahWithYehFinalForm => 'ﶾ',
            ArabicPresentationFormsA::ArabicLigatureHahWithJeemWithYehFinalForm => 'ﶿ',
            ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithYehFinalForm => 'ﷀ',
            ArabicPresentationFormsA::ArabicLigatureFehWithMeemWithYehFinalForm => 'ﷁ',
            ArabicPresentationFormsA::ArabicLigatureBehWithHahWithYehFinalForm => 'ﷂ',
            ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithMeemInitialForm => 'ﷃ',
            ArabicPresentationFormsA::ArabicLigatureAinWithJeemWithMeemInitialForm => 'ﷄ',
            ArabicPresentationFormsA::ArabicLigatureSadWithMeemWithMeemInitialForm => 'ﷅ',
            ArabicPresentationFormsA::ArabicLigatureSeenWithKhahWithYehFinalForm => 'ﷆ',
            ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithYehFinalForm => 'ﷇ',
            ArabicPresentationFormsA::ArabicLigatureSallaUsedAsKoranicStopSignIsolatedForm => 'ﷰ',
            ArabicPresentationFormsA::ArabicLigatureQalaUsedAsKoranicStopSignIsolatedForm => 'ﷱ',
            ArabicPresentationFormsA::ArabicLigatureAllahIsolatedForm => 'ﷲ',
            ArabicPresentationFormsA::ArabicLigatureAkbarIsolatedForm => 'ﷳ',
            ArabicPresentationFormsA::ArabicLigatureMohammadIsolatedForm => 'ﷴ',
            ArabicPresentationFormsA::ArabicLigatureSalamIsolatedForm => 'ﷵ',
            ArabicPresentationFormsA::ArabicLigatureRasoulIsolatedForm => 'ﷶ',
            ArabicPresentationFormsA::ArabicLigatureAlayheIsolatedForm => 'ﷷ',
            ArabicPresentationFormsA::ArabicLigatureWasallamIsolatedForm => 'ﷸ',
            ArabicPresentationFormsA::ArabicLigatureSallaIsolatedForm => 'ﷹ',
            ArabicPresentationFormsA::ArabicLigatureSallallahouAlayheWasallam => 'ﷺ',
            ArabicPresentationFormsA::ArabicLigatureJallajalalouhou => 'ﷻ',
            ArabicPresentationFormsA::RialSign => '﷼',
            ArabicPresentationFormsA::ArabicLigatureBismillahArDashRahmanArDashRaheem => '﷽',
        }
    }
}

impl std::convert::TryFrom<char> for ArabicPresentationFormsA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ﭐ' => Ok(ArabicPresentationFormsA::ArabicLetterAlefWaslaIsolatedForm),
            'ﭑ' => Ok(ArabicPresentationFormsA::ArabicLetterAlefWaslaFinalForm),
            'ﭒ' => Ok(ArabicPresentationFormsA::ArabicLetterBeehIsolatedForm),
            'ﭓ' => Ok(ArabicPresentationFormsA::ArabicLetterBeehFinalForm),
            'ﭔ' => Ok(ArabicPresentationFormsA::ArabicLetterBeehInitialForm),
            'ﭕ' => Ok(ArabicPresentationFormsA::ArabicLetterBeehMedialForm),
            'ﭖ' => Ok(ArabicPresentationFormsA::ArabicLetterPehIsolatedForm),
            'ﭗ' => Ok(ArabicPresentationFormsA::ArabicLetterPehFinalForm),
            'ﭘ' => Ok(ArabicPresentationFormsA::ArabicLetterPehInitialForm),
            'ﭙ' => Ok(ArabicPresentationFormsA::ArabicLetterPehMedialForm),
            'ﭚ' => Ok(ArabicPresentationFormsA::ArabicLetterBehehIsolatedForm),
            'ﭛ' => Ok(ArabicPresentationFormsA::ArabicLetterBehehFinalForm),
            'ﭜ' => Ok(ArabicPresentationFormsA::ArabicLetterBehehInitialForm),
            'ﭝ' => Ok(ArabicPresentationFormsA::ArabicLetterBehehMedialForm),
            'ﭞ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehehIsolatedForm),
            'ﭟ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehehFinalForm),
            'ﭠ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehehInitialForm),
            'ﭡ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehehMedialForm),
            'ﭢ' => Ok(ArabicPresentationFormsA::ArabicLetterTehehIsolatedForm),
            'ﭣ' => Ok(ArabicPresentationFormsA::ArabicLetterTehehFinalForm),
            'ﭤ' => Ok(ArabicPresentationFormsA::ArabicLetterTehehInitialForm),
            'ﭥ' => Ok(ArabicPresentationFormsA::ArabicLetterTehehMedialForm),
            'ﭦ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehIsolatedForm),
            'ﭧ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehFinalForm),
            'ﭨ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehInitialForm),
            'ﭩ' => Ok(ArabicPresentationFormsA::ArabicLetterTtehMedialForm),
            'ﭪ' => Ok(ArabicPresentationFormsA::ArabicLetterVehIsolatedForm),
            'ﭫ' => Ok(ArabicPresentationFormsA::ArabicLetterVehFinalForm),
            'ﭬ' => Ok(ArabicPresentationFormsA::ArabicLetterVehInitialForm),
            'ﭭ' => Ok(ArabicPresentationFormsA::ArabicLetterVehMedialForm),
            'ﭮ' => Ok(ArabicPresentationFormsA::ArabicLetterPehehIsolatedForm),
            'ﭯ' => Ok(ArabicPresentationFormsA::ArabicLetterPehehFinalForm),
            'ﭰ' => Ok(ArabicPresentationFormsA::ArabicLetterPehehInitialForm),
            'ﭱ' => Ok(ArabicPresentationFormsA::ArabicLetterPehehMedialForm),
            'ﭲ' => Ok(ArabicPresentationFormsA::ArabicLetterDyehIsolatedForm),
            'ﭳ' => Ok(ArabicPresentationFormsA::ArabicLetterDyehFinalForm),
            'ﭴ' => Ok(ArabicPresentationFormsA::ArabicLetterDyehInitialForm),
            'ﭵ' => Ok(ArabicPresentationFormsA::ArabicLetterDyehMedialForm),
            'ﭶ' => Ok(ArabicPresentationFormsA::ArabicLetterNyehIsolatedForm),
            'ﭷ' => Ok(ArabicPresentationFormsA::ArabicLetterNyehFinalForm),
            'ﭸ' => Ok(ArabicPresentationFormsA::ArabicLetterNyehInitialForm),
            'ﭹ' => Ok(ArabicPresentationFormsA::ArabicLetterNyehMedialForm),
            'ﭺ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehIsolatedForm),
            'ﭻ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehFinalForm),
            'ﭼ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehInitialForm),
            'ﭽ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehMedialForm),
            'ﭾ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehehIsolatedForm),
            'ﭿ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehehFinalForm),
            'ﮀ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehehInitialForm),
            'ﮁ' => Ok(ArabicPresentationFormsA::ArabicLetterTchehehMedialForm),
            'ﮂ' => Ok(ArabicPresentationFormsA::ArabicLetterDdahalIsolatedForm),
            'ﮃ' => Ok(ArabicPresentationFormsA::ArabicLetterDdahalFinalForm),
            'ﮄ' => Ok(ArabicPresentationFormsA::ArabicLetterDahalIsolatedForm),
            'ﮅ' => Ok(ArabicPresentationFormsA::ArabicLetterDahalFinalForm),
            'ﮆ' => Ok(ArabicPresentationFormsA::ArabicLetterDulIsolatedForm),
            'ﮇ' => Ok(ArabicPresentationFormsA::ArabicLetterDulFinalForm),
            'ﮈ' => Ok(ArabicPresentationFormsA::ArabicLetterDdalIsolatedForm),
            'ﮉ' => Ok(ArabicPresentationFormsA::ArabicLetterDdalFinalForm),
            'ﮊ' => Ok(ArabicPresentationFormsA::ArabicLetterJehIsolatedForm),
            'ﮋ' => Ok(ArabicPresentationFormsA::ArabicLetterJehFinalForm),
            'ﮌ' => Ok(ArabicPresentationFormsA::ArabicLetterRrehIsolatedForm),
            'ﮍ' => Ok(ArabicPresentationFormsA::ArabicLetterRrehFinalForm),
            'ﮎ' => Ok(ArabicPresentationFormsA::ArabicLetterKehehIsolatedForm),
            'ﮏ' => Ok(ArabicPresentationFormsA::ArabicLetterKehehFinalForm),
            'ﮐ' => Ok(ArabicPresentationFormsA::ArabicLetterKehehInitialForm),
            'ﮑ' => Ok(ArabicPresentationFormsA::ArabicLetterKehehMedialForm),
            'ﮒ' => Ok(ArabicPresentationFormsA::ArabicLetterGafIsolatedForm),
            'ﮓ' => Ok(ArabicPresentationFormsA::ArabicLetterGafFinalForm),
            'ﮔ' => Ok(ArabicPresentationFormsA::ArabicLetterGafInitialForm),
            'ﮕ' => Ok(ArabicPresentationFormsA::ArabicLetterGafMedialForm),
            'ﮖ' => Ok(ArabicPresentationFormsA::ArabicLetterGuehIsolatedForm),
            'ﮗ' => Ok(ArabicPresentationFormsA::ArabicLetterGuehFinalForm),
            'ﮘ' => Ok(ArabicPresentationFormsA::ArabicLetterGuehInitialForm),
            'ﮙ' => Ok(ArabicPresentationFormsA::ArabicLetterGuehMedialForm),
            'ﮚ' => Ok(ArabicPresentationFormsA::ArabicLetterNgoehIsolatedForm),
            'ﮛ' => Ok(ArabicPresentationFormsA::ArabicLetterNgoehFinalForm),
            'ﮜ' => Ok(ArabicPresentationFormsA::ArabicLetterNgoehInitialForm),
            'ﮝ' => Ok(ArabicPresentationFormsA::ArabicLetterNgoehMedialForm),
            'ﮞ' => Ok(ArabicPresentationFormsA::ArabicLetterNoonGhunnaIsolatedForm),
            'ﮟ' => Ok(ArabicPresentationFormsA::ArabicLetterNoonGhunnaFinalForm),
            'ﮠ' => Ok(ArabicPresentationFormsA::ArabicLetterRnoonIsolatedForm),
            'ﮡ' => Ok(ArabicPresentationFormsA::ArabicLetterRnoonFinalForm),
            'ﮢ' => Ok(ArabicPresentationFormsA::ArabicLetterRnoonInitialForm),
            'ﮣ' => Ok(ArabicPresentationFormsA::ArabicLetterRnoonMedialForm),
            'ﮤ' => Ok(ArabicPresentationFormsA::ArabicLetterHehWithYehAboveIsolatedForm),
            'ﮥ' => Ok(ArabicPresentationFormsA::ArabicLetterHehWithYehAboveFinalForm),
            'ﮦ' => Ok(ArabicPresentationFormsA::ArabicLetterHehGoalIsolatedForm),
            'ﮧ' => Ok(ArabicPresentationFormsA::ArabicLetterHehGoalFinalForm),
            'ﮨ' => Ok(ArabicPresentationFormsA::ArabicLetterHehGoalInitialForm),
            'ﮩ' => Ok(ArabicPresentationFormsA::ArabicLetterHehGoalMedialForm),
            'ﮪ' => Ok(ArabicPresentationFormsA::ArabicLetterHehDoachashmeeIsolatedForm),
            'ﮫ' => Ok(ArabicPresentationFormsA::ArabicLetterHehDoachashmeeFinalForm),
            'ﮬ' => Ok(ArabicPresentationFormsA::ArabicLetterHehDoachashmeeInitialForm),
            'ﮭ' => Ok(ArabicPresentationFormsA::ArabicLetterHehDoachashmeeMedialForm),
            'ﮮ' => Ok(ArabicPresentationFormsA::ArabicLetterYehBarreeIsolatedForm),
            'ﮯ' => Ok(ArabicPresentationFormsA::ArabicLetterYehBarreeFinalForm),
            'ﮰ' => Ok(ArabicPresentationFormsA::ArabicLetterYehBarreeWithHamzaAboveIsolatedForm),
            'ﮱ' => Ok(ArabicPresentationFormsA::ArabicLetterYehBarreeWithHamzaAboveFinalForm),
            '﮲' => Ok(ArabicPresentationFormsA::ArabicSymbolDotAbove),
            '﮳' => Ok(ArabicPresentationFormsA::ArabicSymbolDotBelow),
            '﮴' => Ok(ArabicPresentationFormsA::ArabicSymbolTwoDotsAbove),
            '﮵' => Ok(ArabicPresentationFormsA::ArabicSymbolTwoDotsBelow),
            '﮶' => Ok(ArabicPresentationFormsA::ArabicSymbolThreeDotsAbove),
            '﮷' => Ok(ArabicPresentationFormsA::ArabicSymbolThreeDotsBelow),
            '﮸' => Ok(ArabicPresentationFormsA::ArabicSymbolThreeDotsPointingDownwardsAbove),
            '﮹' => Ok(ArabicPresentationFormsA::ArabicSymbolThreeDotsPointingDownwardsBelow),
            '﮺' => Ok(ArabicPresentationFormsA::ArabicSymbolFourDotsAbove),
            '﮻' => Ok(ArabicPresentationFormsA::ArabicSymbolFourDotsBelow),
            '﮼' => Ok(ArabicPresentationFormsA::ArabicSymbolDoubleVerticalBarBelow),
            '﮽' => Ok(ArabicPresentationFormsA::ArabicSymbolTwoDotsVerticallyAbove),
            '﮾' => Ok(ArabicPresentationFormsA::ArabicSymbolTwoDotsVerticallyBelow),
            '﮿' => Ok(ArabicPresentationFormsA::ArabicSymbolRing),
            '﯀' => Ok(ArabicPresentationFormsA::ArabicSymbolSmallTahAbove),
            '﯁' => Ok(ArabicPresentationFormsA::ArabicSymbolSmallTahBelow),
            'ﯓ' => Ok(ArabicPresentationFormsA::ArabicLetterNgIsolatedForm),
            'ﯔ' => Ok(ArabicPresentationFormsA::ArabicLetterNgFinalForm),
            'ﯕ' => Ok(ArabicPresentationFormsA::ArabicLetterNgInitialForm),
            'ﯖ' => Ok(ArabicPresentationFormsA::ArabicLetterNgMedialForm),
            'ﯗ' => Ok(ArabicPresentationFormsA::ArabicLetterUIsolatedForm),
            'ﯘ' => Ok(ArabicPresentationFormsA::ArabicLetterUFinalForm),
            'ﯙ' => Ok(ArabicPresentationFormsA::ArabicLetterOeIsolatedForm),
            'ﯚ' => Ok(ArabicPresentationFormsA::ArabicLetterOeFinalForm),
            'ﯛ' => Ok(ArabicPresentationFormsA::ArabicLetterYuIsolatedForm),
            'ﯜ' => Ok(ArabicPresentationFormsA::ArabicLetterYuFinalForm),
            'ﯝ' => Ok(ArabicPresentationFormsA::ArabicLetterUWithHamzaAboveIsolatedForm),
            'ﯞ' => Ok(ArabicPresentationFormsA::ArabicLetterVeIsolatedForm),
            'ﯟ' => Ok(ArabicPresentationFormsA::ArabicLetterVeFinalForm),
            'ﯠ' => Ok(ArabicPresentationFormsA::ArabicLetterKirghizOeIsolatedForm),
            'ﯡ' => Ok(ArabicPresentationFormsA::ArabicLetterKirghizOeFinalForm),
            'ﯢ' => Ok(ArabicPresentationFormsA::ArabicLetterKirghizYuIsolatedForm),
            'ﯣ' => Ok(ArabicPresentationFormsA::ArabicLetterKirghizYuFinalForm),
            'ﯤ' => Ok(ArabicPresentationFormsA::ArabicLetterEIsolatedForm),
            'ﯥ' => Ok(ArabicPresentationFormsA::ArabicLetterEFinalForm),
            'ﯦ' => Ok(ArabicPresentationFormsA::ArabicLetterEInitialForm),
            'ﯧ' => Ok(ArabicPresentationFormsA::ArabicLetterEMedialForm),
            'ﯨ' => Ok(ArabicPresentationFormsA::ArabicLetterUighurKazakhKirghizAlefMaksuraInitialForm),
            'ﯩ' => Ok(ArabicPresentationFormsA::ArabicLetterUighurKazakhKirghizAlefMaksuraMedialForm),
            'ﯪ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefIsolatedForm),
            'ﯫ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefFinalForm),
            'ﯬ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAeIsolatedForm),
            'ﯭ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAeFinalForm),
            'ﯮ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithWawIsolatedForm),
            'ﯯ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithWawFinalForm),
            'ﯰ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithUIsolatedForm),
            'ﯱ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithUFinalForm),
            'ﯲ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithOeIsolatedForm),
            'ﯳ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithOeFinalForm),
            'ﯴ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYuIsolatedForm),
            'ﯵ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYuFinalForm),
            'ﯶ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEIsolatedForm),
            'ﯷ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEFinalForm),
            'ﯸ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithEInitialForm),
            'ﯹ' => Ok(ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraIsolatedForm),
            'ﯺ' => Ok(ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraFinalForm),
            'ﯻ' => Ok(ArabicPresentationFormsA::ArabicLigatureUighurKirghizYehWithHamzaAboveWithAlefMaksuraInitialForm),
            'ﯼ' => Ok(ArabicPresentationFormsA::ArabicLetterFarsiYehIsolatedForm),
            'ﯽ' => Ok(ArabicPresentationFormsA::ArabicLetterFarsiYehFinalForm),
            'ﯾ' => Ok(ArabicPresentationFormsA::ArabicLetterFarsiYehInitialForm),
            'ﯿ' => Ok(ArabicPresentationFormsA::ArabicLetterFarsiYehMedialForm),
            'ﰀ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithJeemIsolatedForm),
            'ﰁ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHahIsolatedForm),
            'ﰂ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemIsolatedForm),
            'ﰃ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefMaksuraIsolatedForm),
            'ﰄ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYehIsolatedForm),
            'ﰅ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithJeemIsolatedForm),
            'ﰆ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithHahIsolatedForm),
            'ﰇ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithKhahIsolatedForm),
            'ﰈ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithMeemIsolatedForm),
            'ﰉ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithAlefMaksuraIsolatedForm),
            'ﰊ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithYehIsolatedForm),
            'ﰋ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithJeemIsolatedForm),
            'ﰌ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHahIsolatedForm),
            'ﰍ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithKhahIsolatedForm),
            'ﰎ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemIsolatedForm),
            'ﰏ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithAlefMaksuraIsolatedForm),
            'ﰐ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithYehIsolatedForm),
            'ﰑ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithJeemIsolatedForm),
            'ﰒ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithMeemIsolatedForm),
            'ﰓ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithAlefMaksuraIsolatedForm),
            'ﰔ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithYehIsolatedForm),
            'ﰕ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithHahIsolatedForm),
            'ﰖ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemIsolatedForm),
            'ﰗ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithJeemIsolatedForm),
            'ﰘ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithMeemIsolatedForm),
            'ﰙ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithJeemIsolatedForm),
            'ﰚ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithHahIsolatedForm),
            'ﰛ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithMeemIsolatedForm),
            'ﰜ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithJeemIsolatedForm),
            'ﰝ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHahIsolatedForm),
            'ﰞ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithKhahIsolatedForm),
            'ﰟ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemIsolatedForm),
            'ﰠ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithHahIsolatedForm),
            'ﰡ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithMeemIsolatedForm),
            'ﰢ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithJeemIsolatedForm),
            'ﰣ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithHahIsolatedForm),
            'ﰤ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithKhahIsolatedForm),
            'ﰥ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithMeemIsolatedForm),
            'ﰦ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithHahIsolatedForm),
            'ﰧ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemIsolatedForm),
            'ﰨ' => Ok(ArabicPresentationFormsA::ArabicLigatureZahWithMeemIsolatedForm),
            'ﰩ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithJeemIsolatedForm),
            'ﰪ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemIsolatedForm),
            'ﰫ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithJeemIsolatedForm),
            'ﰬ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithMeemIsolatedForm),
            'ﰭ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithJeemIsolatedForm),
            'ﰮ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithHahIsolatedForm),
            'ﰯ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithKhahIsolatedForm),
            'ﰰ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithMeemIsolatedForm),
            'ﰱ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithAlefMaksuraIsolatedForm),
            'ﰲ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithYehIsolatedForm),
            'ﰳ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithHahIsolatedForm),
            'ﰴ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemIsolatedForm),
            'ﰵ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithAlefMaksuraIsolatedForm),
            'ﰶ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithYehIsolatedForm),
            'ﰷ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithAlefIsolatedForm),
            'ﰸ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithJeemIsolatedForm),
            'ﰹ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithHahIsolatedForm),
            'ﰺ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithKhahIsolatedForm),
            'ﰻ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithLamIsolatedForm),
            'ﰼ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemIsolatedForm),
            'ﰽ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithAlefMaksuraIsolatedForm),
            'ﰾ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithYehIsolatedForm),
            'ﰿ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemIsolatedForm),
            'ﱀ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahIsolatedForm),
            'ﱁ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithKhahIsolatedForm),
            'ﱂ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemIsolatedForm),
            'ﱃ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithAlefMaksuraIsolatedForm),
            'ﱄ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithYehIsolatedForm),
            'ﱅ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemIsolatedForm),
            'ﱆ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithHahIsolatedForm),
            'ﱇ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithKhahIsolatedForm),
            'ﱈ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithMeemIsolatedForm),
            'ﱉ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithAlefMaksuraIsolatedForm),
            'ﱊ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithYehIsolatedForm),
            'ﱋ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemIsolatedForm),
            'ﱌ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHahIsolatedForm),
            'ﱍ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithKhahIsolatedForm),
            'ﱎ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemIsolatedForm),
            'ﱏ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithAlefMaksuraIsolatedForm),
            'ﱐ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithYehIsolatedForm),
            'ﱑ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithJeemIsolatedForm),
            'ﱒ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithMeemIsolatedForm),
            'ﱓ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithAlefMaksuraIsolatedForm),
            'ﱔ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithYehIsolatedForm),
            'ﱕ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithJeemIsolatedForm),
            'ﱖ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHahIsolatedForm),
            'ﱗ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithKhahIsolatedForm),
            'ﱘ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemIsolatedForm),
            'ﱙ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithAlefMaksuraIsolatedForm),
            'ﱚ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithYehIsolatedForm),
            'ﱛ' => Ok(ArabicPresentationFormsA::ArabicLigatureThalWithSuperscriptAlefIsolatedForm),
            'ﱜ' => Ok(ArabicPresentationFormsA::ArabicLigatureRehWithSuperscriptAlefIsolatedForm),
            'ﱝ' => Ok(ArabicPresentationFormsA::ArabicLigatureAlefMaksuraWithSuperscriptAlefIsolatedForm),
            'ﱞ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithDammatanIsolatedForm),
            'ﱟ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithKasratanIsolatedForm),
            'ﱠ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithFathaIsolatedForm),
            'ﱡ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithDammaIsolatedForm),
            'ﱢ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithKasraIsolatedForm),
            'ﱣ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithSuperscriptAlefIsolatedForm),
            'ﱤ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithRehFinalForm),
            'ﱥ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithZainFinalForm),
            'ﱦ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemFinalForm),
            'ﱧ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithNoonFinalForm),
            'ﱨ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithAlefMaksuraFinalForm),
            'ﱩ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithYehFinalForm),
            'ﱪ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithRehFinalForm),
            'ﱫ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithZainFinalForm),
            'ﱬ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithMeemFinalForm),
            'ﱭ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithNoonFinalForm),
            'ﱮ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithAlefMaksuraFinalForm),
            'ﱯ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithYehFinalForm),
            'ﱰ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithRehFinalForm),
            'ﱱ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithZainFinalForm),
            'ﱲ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemFinalForm),
            'ﱳ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithNoonFinalForm),
            'ﱴ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithAlefMaksuraFinalForm),
            'ﱵ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithYehFinalForm),
            'ﱶ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithRehFinalForm),
            'ﱷ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithZainFinalForm),
            'ﱸ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithMeemFinalForm),
            'ﱹ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithNoonFinalForm),
            'ﱺ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithAlefMaksuraFinalForm),
            'ﱻ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithYehFinalForm),
            'ﱼ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithAlefMaksuraFinalForm),
            'ﱽ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithYehFinalForm),
            'ﱾ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithAlefMaksuraFinalForm),
            'ﱿ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithYehFinalForm),
            'ﲀ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithAlefFinalForm),
            'ﲁ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithLamFinalForm),
            'ﲂ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemFinalForm),
            'ﲃ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithAlefMaksuraFinalForm),
            'ﲄ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithYehFinalForm),
            'ﲅ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemFinalForm),
            'ﲆ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithAlefMaksuraFinalForm),
            'ﲇ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithYehFinalForm),
            'ﲈ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithAlefFinalForm),
            'ﲉ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithMeemFinalForm),
            'ﲊ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithRehFinalForm),
            'ﲋ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithZainFinalForm),
            'ﲌ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemFinalForm),
            'ﲍ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithNoonFinalForm),
            'ﲎ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithAlefMaksuraFinalForm),
            'ﲏ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithYehFinalForm),
            'ﲐ' => Ok(ArabicPresentationFormsA::ArabicLigatureAlefMaksuraWithSuperscriptAlefFinalForm),
            'ﲑ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithRehFinalForm),
            'ﲒ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithZainFinalForm),
            'ﲓ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemFinalForm),
            'ﲔ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithNoonFinalForm),
            'ﲕ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithAlefMaksuraFinalForm),
            'ﲖ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithYehFinalForm),
            'ﲗ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithJeemInitialForm),
            'ﲘ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHahInitialForm),
            'ﲙ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithKhahInitialForm),
            'ﲚ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemInitialForm),
            'ﲛ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHehInitialForm),
            'ﲜ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithJeemInitialForm),
            'ﲝ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithHahInitialForm),
            'ﲞ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithKhahInitialForm),
            'ﲟ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithMeemInitialForm),
            'ﲠ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithHehInitialForm),
            'ﲡ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithJeemInitialForm),
            'ﲢ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHahInitialForm),
            'ﲣ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithKhahInitialForm),
            'ﲤ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemInitialForm),
            'ﲥ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHehInitialForm),
            'ﲦ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithMeemInitialForm),
            'ﲧ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithHahInitialForm),
            'ﲨ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemInitialForm),
            'ﲩ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithJeemInitialForm),
            'ﲪ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithMeemInitialForm),
            'ﲫ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithJeemInitialForm),
            'ﲬ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithMeemInitialForm),
            'ﲭ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithJeemInitialForm),
            'ﲮ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHahInitialForm),
            'ﲯ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithKhahInitialForm),
            'ﲰ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemInitialForm),
            'ﲱ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithHahInitialForm),
            'ﲲ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithKhahInitialForm),
            'ﲳ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithMeemInitialForm),
            'ﲴ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithJeemInitialForm),
            'ﲵ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithHahInitialForm),
            'ﲶ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithKhahInitialForm),
            'ﲷ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithMeemInitialForm),
            'ﲸ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithHahInitialForm),
            'ﲹ' => Ok(ArabicPresentationFormsA::ArabicLigatureZahWithMeemInitialForm),
            'ﲺ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithJeemInitialForm),
            'ﲻ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemInitialForm),
            'ﲼ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithJeemInitialForm),
            'ﲽ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithMeemInitialForm),
            'ﲾ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithJeemInitialForm),
            'ﲿ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithHahInitialForm),
            'ﳀ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithKhahInitialForm),
            'ﳁ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithMeemInitialForm),
            'ﳂ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithHahInitialForm),
            'ﳃ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemInitialForm),
            'ﳄ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithJeemInitialForm),
            'ﳅ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithHahInitialForm),
            'ﳆ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithKhahInitialForm),
            'ﳇ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithLamInitialForm),
            'ﳈ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemInitialForm),
            'ﳉ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemInitialForm),
            'ﳊ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahInitialForm),
            'ﳋ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithKhahInitialForm),
            'ﳌ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemInitialForm),
            'ﳍ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHehInitialForm),
            'ﳎ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemInitialForm),
            'ﳏ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithHahInitialForm),
            'ﳐ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithKhahInitialForm),
            'ﳑ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithMeemInitialForm),
            'ﳒ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemInitialForm),
            'ﳓ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHahInitialForm),
            'ﳔ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithKhahInitialForm),
            'ﳕ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemInitialForm),
            'ﳖ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHehInitialForm),
            'ﳗ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithJeemInitialForm),
            'ﳘ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithMeemInitialForm),
            'ﳙ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithSuperscriptAlefInitialForm),
            'ﳚ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithJeemInitialForm),
            'ﳛ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHahInitialForm),
            'ﳜ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithKhahInitialForm),
            'ﳝ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemInitialForm),
            'ﳞ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHehInitialForm),
            'ﳟ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithMeemMedialForm),
            'ﳠ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHamzaAboveWithHehMedialForm),
            'ﳡ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithMeemMedialForm),
            'ﳢ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithHehMedialForm),
            'ﳣ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemMedialForm),
            'ﳤ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHehMedialForm),
            'ﳥ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithMeemMedialForm),
            'ﳦ' => Ok(ArabicPresentationFormsA::ArabicLigatureThehWithHehMedialForm),
            'ﳧ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemMedialForm),
            'ﳨ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHehMedialForm),
            'ﳩ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemMedialForm),
            'ﳪ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHehMedialForm),
            'ﳫ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithLamMedialForm),
            'ﳬ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemMedialForm),
            'ﳭ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemMedialForm),
            'ﳮ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemMedialForm),
            'ﳯ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHehMedialForm),
            'ﳰ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemMedialForm),
            'ﳱ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHehMedialForm),
            'ﳲ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithFathaMedialForm),
            'ﳳ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithDammaMedialForm),
            'ﳴ' => Ok(ArabicPresentationFormsA::ArabicLigatureShaddaWithKasraMedialForm),
            'ﳵ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithAlefMaksuraIsolatedForm),
            'ﳶ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithYehIsolatedForm),
            'ﳷ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithAlefMaksuraIsolatedForm),
            'ﳸ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithYehIsolatedForm),
            'ﳹ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithAlefMaksuraIsolatedForm),
            'ﳺ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithYehIsolatedForm),
            'ﳻ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithAlefMaksuraIsolatedForm),
            'ﳼ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithYehIsolatedForm),
            'ﳽ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithAlefMaksuraIsolatedForm),
            'ﳾ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithYehIsolatedForm),
            'ﳿ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithAlefMaksuraIsolatedForm),
            'ﴀ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithYehIsolatedForm),
            'ﴁ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithAlefMaksuraIsolatedForm),
            'ﴂ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithYehIsolatedForm),
            'ﴃ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithAlefMaksuraIsolatedForm),
            'ﴄ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithYehIsolatedForm),
            'ﴅ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithAlefMaksuraIsolatedForm),
            'ﴆ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithYehIsolatedForm),
            'ﴇ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithAlefMaksuraIsolatedForm),
            'ﴈ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithYehIsolatedForm),
            'ﴉ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithJeemIsolatedForm),
            'ﴊ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahIsolatedForm),
            'ﴋ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithKhahIsolatedForm),
            'ﴌ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemIsolatedForm),
            'ﴍ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithRehIsolatedForm),
            'ﴎ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithRehIsolatedForm),
            'ﴏ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithRehIsolatedForm),
            'ﴐ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithRehIsolatedForm),
            'ﴑ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithAlefMaksuraFinalForm),
            'ﴒ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithYehFinalForm),
            'ﴓ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithAlefMaksuraFinalForm),
            'ﴔ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithYehFinalForm),
            'ﴕ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithAlefMaksuraFinalForm),
            'ﴖ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithYehFinalForm),
            'ﴗ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithAlefMaksuraFinalForm),
            'ﴘ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithYehFinalForm),
            'ﴙ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithAlefMaksuraFinalForm),
            'ﴚ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithYehFinalForm),
            'ﴛ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithAlefMaksuraFinalForm),
            'ﴜ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithYehFinalForm),
            'ﴝ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithAlefMaksuraFinalForm),
            'ﴞ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithYehFinalForm),
            'ﴟ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithAlefMaksuraFinalForm),
            'ﴠ' => Ok(ArabicPresentationFormsA::ArabicLigatureKhahWithYehFinalForm),
            'ﴡ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithAlefMaksuraFinalForm),
            'ﴢ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithYehFinalForm),
            'ﴣ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithAlefMaksuraFinalForm),
            'ﴤ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithYehFinalForm),
            'ﴥ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithJeemFinalForm),
            'ﴦ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahFinalForm),
            'ﴧ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithKhahFinalForm),
            'ﴨ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemFinalForm),
            'ﴩ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithRehFinalForm),
            'ﴪ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithRehFinalForm),
            'ﴫ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithRehFinalForm),
            'ﴬ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithRehFinalForm),
            'ﴭ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithJeemInitialForm),
            'ﴮ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahInitialForm),
            'ﴯ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithKhahInitialForm),
            'ﴰ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemInitialForm),
            'ﴱ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHehInitialForm),
            'ﴲ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHehInitialForm),
            'ﴳ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemInitialForm),
            'ﴴ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithJeemMedialForm),
            'ﴵ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHahMedialForm),
            'ﴶ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithKhahMedialForm),
            'ﴷ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithJeemMedialForm),
            'ﴸ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahMedialForm),
            'ﴹ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithKhahMedialForm),
            'ﴺ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemMedialForm),
            'ﴻ' => Ok(ArabicPresentationFormsA::ArabicLigatureZahWithMeemMedialForm),
            'ﴼ' => Ok(ArabicPresentationFormsA::ArabicLigatureAlefWithFathatanFinalForm),
            'ﴽ' => Ok(ArabicPresentationFormsA::ArabicLigatureAlefWithFathatanIsolatedForm),
            '﴾' => Ok(ArabicPresentationFormsA::OrnateLeftParenthesis),
            '﴿' => Ok(ArabicPresentationFormsA::OrnateRightParenthesis),
            'ﵐ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithMeemInitialForm),
            'ﵑ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHahWithJeemFinalForm),
            'ﵒ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHahWithJeemInitialForm),
            'ﵓ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithHahWithMeemInitialForm),
            'ﵔ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithMeemInitialForm),
            'ﵕ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithJeemInitialForm),
            'ﵖ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithHahInitialForm),
            'ﵗ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithKhahInitialForm),
            'ﵘ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithHahFinalForm),
            'ﵙ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithHahInitialForm),
            'ﵚ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithMeemWithYehFinalForm),
            'ﵛ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithMeemWithAlefMaksuraFinalForm),
            'ﵜ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithHahWithJeemInitialForm),
            'ﵝ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithJeemWithHahInitialForm),
            'ﵞ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithJeemWithAlefMaksuraFinalForm),
            'ﵟ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithHahFinalForm),
            'ﵠ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithHahInitialForm),
            'ﵡ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithJeemInitialForm),
            'ﵢ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithMeemFinalForm),
            'ﵣ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithMeemWithMeemInitialForm),
            'ﵤ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithHahWithHahFinalForm),
            'ﵥ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithHahWithHahInitialForm),
            'ﵦ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithMeemWithMeemFinalForm),
            'ﵧ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithMeemFinalForm),
            'ﵨ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithMeemInitialForm),
            'ﵩ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithJeemWithYehFinalForm),
            'ﵪ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithKhahFinalForm),
            'ﵫ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithKhahInitialForm),
            'ﵬ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithMeemFinalForm),
            'ﵭ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithMeemWithMeemInitialForm),
            'ﵮ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithHahWithAlefMaksuraFinalForm),
            'ﵯ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithKhahWithMeemFinalForm),
            'ﵰ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithKhahWithMeemInitialForm),
            'ﵱ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithHahFinalForm),
            'ﵲ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithHahInitialForm),
            'ﵳ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithMeemInitialForm),
            'ﵴ' => Ok(ArabicPresentationFormsA::ArabicLigatureTahWithMeemWithYehFinalForm),
            'ﵵ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithJeemWithMeemFinalForm),
            'ﵶ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithMeemFinalForm),
            'ﵷ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithMeemInitialForm),
            'ﵸ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithAlefMaksuraFinalForm),
            'ﵹ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithMeemFinalForm),
            'ﵺ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithYehFinalForm),
            'ﵻ' => Ok(ArabicPresentationFormsA::ArabicLigatureGhainWithMeemWithAlefMaksuraFinalForm),
            'ﵼ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithKhahWithMeemFinalForm),
            'ﵽ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithKhahWithMeemInitialForm),
            'ﵾ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithHahFinalForm),
            'ﵿ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithMeemFinalForm),
            'ﶀ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahWithMeemFinalForm),
            'ﶁ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahWithYehFinalForm),
            'ﶂ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahWithAlefMaksuraFinalForm),
            'ﶃ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithJeemInitialForm),
            'ﶄ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithJeemFinalForm),
            'ﶅ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithKhahWithMeemFinalForm),
            'ﶆ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithKhahWithMeemInitialForm),
            'ﶇ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithHahFinalForm),
            'ﶈ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithHahInitialForm),
            'ﶉ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithJeemInitialForm),
            'ﶊ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithMeemInitialForm),
            'ﶋ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithHahWithYehFinalForm),
            'ﶌ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithHahInitialForm),
            'ﶍ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithMeemInitialForm),
            'ﶎ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithJeemInitialForm),
            'ﶏ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithMeemInitialForm),
            'ﶒ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithKhahInitialForm),
            'ﶓ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithMeemWithJeemInitialForm),
            'ﶔ' => Ok(ArabicPresentationFormsA::ArabicLigatureHehWithMeemWithMeemInitialForm),
            'ﶕ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithMeemInitialForm),
            'ﶖ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithAlefMaksuraFinalForm),
            'ﶗ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithMeemFinalForm),
            'ﶘ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithMeemInitialForm),
            'ﶙ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithAlefMaksuraFinalForm),
            'ﶚ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemWithYehFinalForm),
            'ﶛ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithMeemWithAlefMaksuraFinalForm),
            'ﶜ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithMeemFinalForm),
            'ﶝ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithMeemInitialForm),
            'ﶞ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithKhahWithYehFinalForm),
            'ﶟ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithYehFinalForm),
            'ﶠ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithJeemWithAlefMaksuraFinalForm),
            'ﶡ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithYehFinalForm),
            'ﶢ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithKhahWithAlefMaksuraFinalForm),
            'ﶣ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithYehFinalForm),
            'ﶤ' => Ok(ArabicPresentationFormsA::ArabicLigatureTehWithMeemWithAlefMaksuraFinalForm),
            'ﶥ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithYehFinalForm),
            'ﶦ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithHahWithAlefMaksuraFinalForm),
            'ﶧ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithMeemWithAlefMaksuraFinalForm),
            'ﶨ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithKhahWithAlefMaksuraFinalForm),
            'ﶩ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithHahWithYehFinalForm),
            'ﶪ' => Ok(ArabicPresentationFormsA::ArabicLigatureSheenWithHahWithYehFinalForm),
            'ﶫ' => Ok(ArabicPresentationFormsA::ArabicLigatureDadWithHahWithYehFinalForm),
            'ﶬ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithYehFinalForm),
            'ﶭ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithMeemWithYehFinalForm),
            'ﶮ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithHahWithYehFinalForm),
            'ﶯ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithJeemWithYehFinalForm),
            'ﶰ' => Ok(ArabicPresentationFormsA::ArabicLigatureYehWithMeemWithYehFinalForm),
            'ﶱ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithMeemWithYehFinalForm),
            'ﶲ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithYehFinalForm),
            'ﶳ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithHahWithYehFinalForm),
            'ﶴ' => Ok(ArabicPresentationFormsA::ArabicLigatureQafWithMeemWithHahInitialForm),
            'ﶵ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithHahWithMeemInitialForm),
            'ﶶ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithMeemWithYehFinalForm),
            'ﶷ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithYehFinalForm),
            'ﶸ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithHahInitialForm),
            'ﶹ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithKhahWithYehFinalForm),
            'ﶺ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithMeemInitialForm),
            'ﶻ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithMeemFinalForm),
            'ﶼ' => Ok(ArabicPresentationFormsA::ArabicLigatureLamWithJeemWithMeemFinalForm),
            'ﶽ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithHahFinalForm),
            'ﶾ' => Ok(ArabicPresentationFormsA::ArabicLigatureJeemWithHahWithYehFinalForm),
            'ﶿ' => Ok(ArabicPresentationFormsA::ArabicLigatureHahWithJeemWithYehFinalForm),
            'ﷀ' => Ok(ArabicPresentationFormsA::ArabicLigatureMeemWithJeemWithYehFinalForm),
            'ﷁ' => Ok(ArabicPresentationFormsA::ArabicLigatureFehWithMeemWithYehFinalForm),
            'ﷂ' => Ok(ArabicPresentationFormsA::ArabicLigatureBehWithHahWithYehFinalForm),
            'ﷃ' => Ok(ArabicPresentationFormsA::ArabicLigatureKafWithMeemWithMeemInitialForm),
            'ﷄ' => Ok(ArabicPresentationFormsA::ArabicLigatureAinWithJeemWithMeemInitialForm),
            'ﷅ' => Ok(ArabicPresentationFormsA::ArabicLigatureSadWithMeemWithMeemInitialForm),
            'ﷆ' => Ok(ArabicPresentationFormsA::ArabicLigatureSeenWithKhahWithYehFinalForm),
            'ﷇ' => Ok(ArabicPresentationFormsA::ArabicLigatureNoonWithJeemWithYehFinalForm),
            'ﷰ' => Ok(ArabicPresentationFormsA::ArabicLigatureSallaUsedAsKoranicStopSignIsolatedForm),
            'ﷱ' => Ok(ArabicPresentationFormsA::ArabicLigatureQalaUsedAsKoranicStopSignIsolatedForm),
            'ﷲ' => Ok(ArabicPresentationFormsA::ArabicLigatureAllahIsolatedForm),
            'ﷳ' => Ok(ArabicPresentationFormsA::ArabicLigatureAkbarIsolatedForm),
            'ﷴ' => Ok(ArabicPresentationFormsA::ArabicLigatureMohammadIsolatedForm),
            'ﷵ' => Ok(ArabicPresentationFormsA::ArabicLigatureSalamIsolatedForm),
            'ﷶ' => Ok(ArabicPresentationFormsA::ArabicLigatureRasoulIsolatedForm),
            'ﷷ' => Ok(ArabicPresentationFormsA::ArabicLigatureAlayheIsolatedForm),
            'ﷸ' => Ok(ArabicPresentationFormsA::ArabicLigatureWasallamIsolatedForm),
            'ﷹ' => Ok(ArabicPresentationFormsA::ArabicLigatureSallaIsolatedForm),
            'ﷺ' => Ok(ArabicPresentationFormsA::ArabicLigatureSallallahouAlayheWasallam),
            'ﷻ' => Ok(ArabicPresentationFormsA::ArabicLigatureJallajalalouhou),
            '﷼' => Ok(ArabicPresentationFormsA::RialSign),
            '﷽' => Ok(ArabicPresentationFormsA::ArabicLigatureBismillahArDashRahmanArDashRaheem),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ArabicPresentationFormsA {
    fn into(self) -> u32 {
        let c: char = self.into();
        let hex = c
            .escape_unicode()
            .to_string()
            .replace("\\u{", "")
            .replace("}", "");
        u32::from_str_radix(&hex, 16).unwrap()
    }
}

impl std::convert::TryFrom<u32> for ArabicPresentationFormsA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ArabicPresentationFormsA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ArabicPresentationFormsA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ArabicPresentationFormsA::ArabicLetterAlefWaslaIsolatedForm
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ArabicPresentationFormsA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
