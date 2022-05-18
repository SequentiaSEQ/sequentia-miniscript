// Miniscript
// Written in 2019 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Example: Verifying a signed transaction.

extern crate elements_miniscript as miniscript;

use std::str::FromStr;

use elements::encode::Decodable;
use elements::{confidential, secp256k1_zkp};

use crate::miniscript::interpreter::KeySigPair; // secp256k1 re-exported from rust-bitcoin
fn main() {
    // some random liquid tx from mempool(Dec 3rd 2020)
    // txid: f23c8973027aa8c1e86580a729833914f5b1fa710367db07f1f5515aa3729f16
    let tx_bytes : Vec<u8> = elements::hashes::hex::FromHex::from_hex(
            "02000000010232228caa1ed022bba919d55646169e6094fcff7580cfd51767c52ad1f1f7be1501000000171600142e7c87cd55d9163b58a0e98cb0c076e8fbc916c5feffffff28b3920a83b2507ca5f3993c84cf7b69021be6887e86e1ec8855ac432199de6b0100000017160014edfc058e1c059532d7aa90cfd47d1bf4f7d2d9b3feffffff030ae977a025f8432195181d77bc6523b7769b8b4d4c3cf8f5a45bdb940c7dfb561e0878ac4e6ab21b09f64a262d0342e88928cfa68bc9e8d004109f9caca1b305d37e0217abccf88d3488db1e40f79dfaf14ca816431441095bcb2570c9a0dcca01456c17a914b5687f4af9ba5dfb00ed22cb32a857e607bc844d870b892d93c6accf6253edd8340a6c27caaee3d5dc067e626d3c1647dd895299404a0936e562586398b4e95fb34c108afd4a9de4268bd5cd41749833e7a9265e383a0e02658aa567bb9656cd06b563855dfd63fb3e8b37b7991f86895c536af5692e819d17a9149f03346fa3e2a4fbeed0e5c379f2f2ba28025bc187016d521c38ec1ea15734ae22b7c46064412829c0d0579f0a713d1c04ede979026f0100000000000001060000e6bf1000000002473044022020d643f80c3cab90a76dfca198bdba119db97cf4fd8d8adfa2609cdbcf594f570220324c54a8e3170d06cea60511e5e8a84d1f41b21fc252181d69aa8b7d190c26a90121029e831ee2345cc698c730b713b82b99e8c16ffe5ddd51c9b36607bb1fa68b1395000000024730440220645f22a912344b4f8d165814c3513e9bfa06d42066fa4d392f783a57444eff0202200b98199177d9e13c9f3872f433154be430e82cc2e312791a47bea2a91167ff4c01210303a640e79b1e941f2d7867e4eabffd79f30daaabc80fe1c4d974775fc29339810063020003c7b8cf924dff6cef0c9f9b507abc741e431e1a793206b5fca623748f583e6ad3f3f7a4fd1b27d78bf3b4debc9a2c849b5f85722cab872a9c08f19e6eb59ceef74fcf9a7c38acd6fdb7821c33bc731011b6440e8aae7beff3ee6ffc1009c849bcfd4e10603300000000000000012361c401dc40cd92cf93029e64c20c463d17c6e15d2997ab43e8c4b1c412b99a1a337c3cff34c6e7269580727c27404a131606aa85dd8bdf6d46b6ddb23948a0507e31666b531e753eb8fece006050bc7b7e2e06db86833f9adc4f6b77c7b88537617c3accea246a8fe303a23697801181efa8bbcded7690aeac04b58e32f4cd94279cff059a5c9f679d973007042fa1e5e4268a884dbce7acba614d57e1f7a56f81354779ab353c04806ad280417f656a10bcbc683348608d7f6e08367d9c21318cfe6f03a0a60bb8269d667a3ddd57f77f8774364e6a946a6163896db4a8b7e2446d229d8d3718f9a8d7b57a2dfe43baa79bce705512635c775602404d55a2a61fee93c00f1e0cc466bf2cf0bc070591e228de578bc51a38b1927ae29f0e7daad95380f5be599f50367ae1028e57f2e1d9b35d45f0fd860491c2c03a46500f2fabf3c8c7cc9ede16af01eb3d42633ddb8ef87bc043c136f727fffcb4aebdb19f674864e2d2a20aa0d61f7e635c582a79320ff2dbbe8ee74c4c51ee09e03e89581378fcc4d2a6f44524add57be3d31d3054468662a805fb82bd047e5b52e13415ea92c57532940985ded222477f5bed2bdedbbe1f2471b0a85c32a70b0d07a5c734ce90e62b51babee95c63f2cd5ba7b6716a9ecb2f44e5f30f2bc07ea7fef5257aff812558385b23966608191436cbeb1f9fc6a907c1841e1dfcea0e5719749969d447bbb9b332389b2e046d107a873c1b557319250c6469ea03c7bd2c61e6a31963921f1854c5546479044dc992f09c2e781c782fa7dc116074787495bedd991b530a7a8b49bd0f85e32ea19bdfac28152bf63eef1a8bd9294b51eaba8b5bc4db1d89031e4dafffff9bae4329fce27eac3db83e6d5d22ec4e04850b7af49329828e087f2c998882249cb74234002c6ea20641f6b4dd0199e4d6d932e75cf0e59a07b25724826fdfedae239d8275c992456c53a2d4c469f08120554e86c4693c32284616279eeb048d434288f6f73dec16d3d37ef0ed2a98e5c11cf6ac046137bbd8b76c1ebb00e8804133e0e647751ab6b395cc49c92d5084bbac242cbd8369c8f56548ca92b8a1da586e12f6163d2d0b42b4774e6c8eac23240c51ec79d71dbd1a69441265c6d5e4974e3ef6358b30106649744c9799f6ec1de0cc98929f02be8396c4d807e87a767e3e2903633da774ff39141e14aba0c01fe3331d7b888360722ebb1c3e1fdb1b3226ecd2003cfe2400ec9ef29f431d6e5182eb98c8aa4384b60dca8dd0d28ea52fdb6b181c4788b6d0d2ce9c5dad4dad259a83eda33c21704d4230660f8962c69d290e77e60f16916d5e1bf811eadd054d464fe33232541070371a69b120402a9598ceeb9c8ffe0622c90fcffcf271eda70128ed21c5c1990c3361c11f7f697eb56a32bc15c5e79596274043eb80da990fb40e31a0d8a83d62e53e9d9f3f786de418ee3c9a1adc54ebb43b74db84d550dc8fb64aa5a0947f23d9e07fc370e141217b0766186315ad088c2ccbb426e3a184ad74b5469ff24b596670617bb967aaf8976438415d851bc89a98887e2388161c967b8ae9e5814af518097c4e0604b6ecf755527ade18ae0a50234a95f4b45c1bf68ada13991928dc88b61abf23445ec95d3f06830852bba27e09a7747b1c06a1d4948214be7e0bb47f2c14f6ac02715a250232ed15767da55717adb239149d8021f33bb14316622947d98d30506aaabadf90add3cd639f5e558450642ac7b4457da522efb710124518a0a3f41b6798127d209768cc526ecc9a6f70edbe3c3b9f45536295c2bcccc384b0d13bd6d741e8d1d01ac83e09cf35a9e7ff71ffbe2926d8b3c5d5eb131fe9b023d427c4c8c3e4310df6b3c45c2de0092df59e98176b1bcce17889b3667854785a668e3c08aec571506e5074ddc5e53eba12a5c2adde7a0fb7c1e5b86b1463e7634980985dff039d90abdb0d11cc701bc8122e22000d3eb4ee0ef422a08e85bf13d5bc856e79c0eb00f415a8a76d6d7bcb087b090954e0476c81ad1ba3ef89b8b5338c78a8c0b5b25cda36ff35c307900066fcf775cf71069a1e396b0efc611b21ecde8b5f080252d16690767137307aade91486ff23fd4ebd206d0b9bbe9f2221003967fb76e51925954fd5df40d0c331277c35bc7bd40c05ed32a4cf577ccd5219b113df6454258cf1cdf123e8ffb7fab3debb05ac9f3a3874221bc6d08db4c0e2676257ba26d2b780c6102014f980987c6826cdab6a713cbfeae703af2dd8838378dec58b406ce2e796a560e1bf1f7f4d09f32c0cad9ad2fc3720ce3963273b6768250c3119eb4d9c0c722cf64733432fa76abf07e845d55f089df83dfb784cd7bae06e1fa26b7464667945774f2c61afac025d4f5dcc909cfaf064ff9b622b680eab1e5ba31b8374fd2402ffae1481cb8ef180060c8ac01aac5a4f9df8b5e84b7872cdeda17cc5b37a4438b6c31b917a88336cbbd535f95710f1d4bda8dee8f9613c0e62e9914dbc8f2f812783b7b8a154c74c17ceb1df61880f61a9038ade4a15542a2258d0533988303e1d22910448fa3d8aa29660efc1af8d8c470da4071960ab55a83627da6c1d955e337fff0f18e6e4b0516ef21beabdd3174ffe816ac945ab7c54a60a56898fce13b782820ad6b6e4f72a82122366695b4d5c332490fe8caf98b3e77749ac06fcaa0a1be7e283201e7dd0ec43cb728da2597af4a4d1d20836d1d443c6fd24d57881ec9b9d0af6b99f04511ba2f526c1647f0c67328f7ea7952bbeaf41198ba56ddeca73e8f1e028d64c9e8bc9515aa9f16830add7c705d6aa4fbf20ba5aef213088d6e4ef4453dfe0b0c531ea19ba772969881e5945e353b43ce2940d9b6898855092f6426b32ff0f1037e1d9799e30d93b1b1df71b70bdc52fcd9db0cc87c55894244da38cc6534dae903d84f7b636cac2d26ee2ee3c60a60c9482dcf3ad679936b6fb4e6a16bd3275f6452a0a47dc0f5acdcf9fe17c9fb34ac669ff277964d5d77062d4dc69ffb3f376665972fc8300df0d3eca41c769891013cb14d70b7600300d030e3ad5d6f9394786551857c5beb49c0eab658b6253ad647365912062b1895ceb0e5ddcfbc73034414bb7eef9bc7a201f735d67edfc7c07c89f3ea8a98bdef324f606ed2b4b3989d0d019b73cd8ac9bdf062c53a40c13ca97f848e152756c388bc54d4ec86104962ea8df3a9430ee4f1bf6eb57862faf2cd49e21db5982dfb5386a23b47b4d010b4ad79f600161ba2d5307b9199977493c1e8a696d2aed4b85d7d52074438c76203baaa50bd96f5d2c9063b4e3033f090021c996261344e44aece16bf926614a5163b70d7f6a03d13462e0858b1911763700503b96505f471bf084d966dd7cd8603e3ffb9b22c572526526d8568e6bb55fa77bfe334ffee4edab2a58719aab70e9e8e8102eadb5391d6a188006a42daee2b2ca71da99b80360fadc4710e7a33d8c3173cc69b5824d93b5021718a276b0ab706029d4378df2fd1183cff056d463a2984c5aa397b9cafd1d3e4a209c04e499130980265d5e2930638bd12656145667bf0dfa09bb61c5c6801d158fead98b3ccc6619c5ff643bc629a57cd6e9b535033d29d4ebdfcff607c50b6ff979fd22fc1a07d796672d98d8b2be6c6c4a4f4911179d18cb29c4c8c490a091d334975033e2ae1a0db38f77ee252f22d4555d0202b39671b6ff29d564cf7b0252ed748cb1a5770bb5aa913d37a7e6e86814e1f34baa5ce474fdaca3bb316cebc4e7893c6452d856a4245434b26b52a37d8aab8f5da3a325e313858e40a9405307de7f4e80d25b3cddea6c75f8595a1c235005a3f0fc948019cfe9e68e3cd0a2affb2f073374d0a03ac55ce9644a92b28409173f763e6a1f7946c1d16ac14a792bd5aea57e404d8a684bdea0b430572d2e5034c949e6c9e74a989d0f91dd740b174e4853e2e693c011d2d29e50bef9fee62079eafe87516448c5a045a16eb569314c9410d4363619ea19aee6460e72440d1cc2a368429c99f10dc8707662d64262113d8eda7842a6233e9ec5c1f7d8c71dc6efedc4bfed53fa9646171caa26daaa0bf4ce50d7b02864ffa0d860014e4d08425f0c1fb840d69a55ec84a833b45dd4572d05d2bcd7dbc6a78974dd2ed0bd52a0343fbd5421c05900da0b7c4e9e42e59183bd2f6e5058482fdd165e346b3ca22f35934538e97110321709394ee446087d97630aa17305fd5d1fd697a4ff3477fc7d02a65afd16e2a5aeccf9402c9dc90f4550140b6bebe6554203e44b9b02472a47c524f4a9554dc25484eb8f173a878a975cdabdfb7993b5e097dc04ef93a7833e51a61b64a6b62743252bf60bf665b9c398688eb24db0d8d2acea848859864f426c08649d29aa7e1d1027318d55624ef750572616ca5b5a118afb83b6e0c872c29a5388f954fcaf8f6d0e67396bd54b9115abe02ffdf52878cd55cb0c5d97cedf41794880ca2b88f960a37d0241ad3e268b3ed142285cfb464c9b0917234df19aa37daa95b935b9f035687d1ac0b7334855e5bf0473175d54c31cf520735d04f5b4ac272cb57999fbb8cf49bd5de6b44616c6a103f53dfc86f874cd847c5adda25c50f3615c0c9c63b34c32142ed62e49f72921fd35753e0ec9333add42838b383684e5246c154cd3ff6310933a36798a4020a027e8f0f951f0dc2dcdb26f0922c69a8dee7183332aa534fa25523469e7bca5917f37fc822b4e7ea5123ef1b8250b2d3dfc9c24ade46394ade3694d528db41108801c3618fd98878e837794023e5ca756e584fd2853829c53380bbb8a281998db43e3ee44e4e6cd08a46c958fee413af351467b7c8275f4c74d4111614eb90b3b10c044c88e1c00f3c2b7959b71bbabb25afa0f564b1449ca585066cd77721fbde395a2c6337be3d546a58498c5f21abf8177c7299fca76e4ae951847858f50fb07cfb10744b770f115e35f57dc70858cafe3a9f66fa406b8a5da94a07699672eefb557fa6adbea267961fd9e4af73a1efdea261a249a573258dae670de24a3d96f36cd146d02370921cacdc155b30a80ad5b43c5db9390ee94e5fcb33b574f82e039371d25fe1128effcb536d5d53727ef341043dd7b07db4c41c117356b3c5379e486f19a4bdef1d3afd4cd0f5fced5a2fba436ee2b84d0b326cc4fe19e69ce05906d3e36c62d9b9a4695bd3ec0d0e5c5e8578dc36094fe879702cf95577a2616244fa3dd7f07c998836793f367d973f640138896f18461b877df9c5a86b046bfab5ac5692d85c5ae632ac51dd726aea59ff471e3f50579204ba6c5868530205faaf81821a57d462ffc697bdd82ba50edd8d3d7973235b57be2d0f9a84ded1d0f3cb68555c2300987f2d14768d2f0214abaf29a06d5d37c5296b5c2db62b2169e1a7bd425cef00db2745d5e246f66191c522f58f0c14fb1d1c8fe6b5b95a45570334125c190a3a9a3e77deb9ae7ca45d973fc64a89d2bb6420286ce01649d5de80ccf89d34bc0c2d9e49d8ec713d2c83286bded0d30a8c42740678c6538d8a7e491b2d1007df7d5037016018c6bd36a21247c13c1cccad168a6a0c2dab4c01185b6d61c31f0f2410b83c16a58d7cdcc0f2522fbe62fb27b1094f77229bff259433e9265d5656931b6d47210bd1e1a66a6c6a3dc9c6a2912a8fc1f05eecf40b751555e569ac2f92d1e5b540037c0cf5c9e658a17fb9dabdd23a087ebb90411f5e75f70bc7786855a2adbebd889d1946ecd6be827e2a624e256c36226ec87f2b92f978be8ffa98169f8dd4947390509d2397322da63757dff25a86f9872414209c50983c570558fc1a630d3caa8f6a59a16cd024f00b2323d2884187ec92feb19e078690f51940229c6076dea7064e71fcc6c55ffd643ccff5b63020003f6512e0b2e6dd0d8cf0a8cde59f4ae60632a902889a27e469b0e844d8477b471a779f720b7650e56645f8a044bebcf4660405a5185e75d13d0c15a221fc707860cfc260b9f9ab3c98e616cc574bd05a494faea98c1e06f9c7ff9643b72692a4afd4e1060330000000000000001cda9b801f96b13ea639d611a15e8f3e899ccf6718b7add1e0309d6c16a3f9b5699cea2fe90fb96147e22f3b025013bb9e7e824614a2c2c2e2e9a420a98b91d71170d89e7aebb414266337a636ec3031021a51701dda9113e9c1ce1291fa9ccab9dd8f30ab98d9c5ba12894af05b42573202c8a5a35d4a49dea9873c2f9aa51ee263a10e49cced91debeff76caca98e07cb7e7705fb647f3ea846cafa1ea4e8449022aedc5210ac18c0a2585a435cc2249cc997aa7e2e180fcee7ee92b1dfee29b3fb125c3d78938b8e6cec68f9fa16643460617013d0bf65b453a124fb843326020adba9ea96bb1832cefd6ac4126329187329b1f31f713b097d34cd2be2c42b70db5e5153a7c5508496f4608c8b7c1c3d8fa7dad87c76c06c278caf0a429d49f4d4fe367894ed493b1e91f331e0533b8364241527adbb867a29bb47fa8e8b5156d5c0ec9042c900f5b9bbe352488b0c671bcd381f090b7fd5ce469d21e01cf5b3f01fcdd51b3fb0eefb4145de49a9444ab161522dbd465169f6d9814b2646142fcc9020aee9742246442a63486e80aa341c81be71f8568080f7d700308f0f79dd8295345da0bd852f5aca16e5fc0e686d7642ef70f46f6d836f3a5bd2b14919fcd5e55ed2dfa8d6920681cc26d1a31f68bf6580c6693b8b877630746cf3a4bea53b4862872941f535345e69a7e7f0108cc72b741a952f4aee5436e918c6fae307277cf20b1fc4035f9235b82c18a24e3159563afd240b0cbe671f2138e6d3b9a4efb5e073b5428e4073a3610c79643b441165121d2d85a311f334b751fac62afec8e1bab82138c5ccd2154f1e00d11e00fae9fc1621ed68e1a32c71b7bd98d5d6328bf7bb3ab5227391eed36335fccd1f83a4b6854a2ddeca1942f8478391be3468693cec629918dd66f00f3a520b495c7f6eaa22aa90a3ee22f29c9ff4e473b739e7a7581411ae966f0d5aa9f157d33066f868a39a0af75199fe886d62c8112ba5242becc11cbbf038c28c41674c643d24448ec7336c98dec3ce09db944476df456e661dec9ffe471243b16888496d17093cb21f0cf5360c884107ed584a24ea151f6f91ddbf92848afe9ca936aa68f992d13aa42e470284454e3b7fe31ec28c33480cbd17d44b941f4f99fe269aa51319cdae2f454d548a41d801061beb6466cf3c27c0efdbf4ea556d57dfd9f52798214f73c3506525426a59820d037285f61465a141e394de9970b6c9b6297ada4d4d65fc5b9921a8f9e6aad8a9ab2d587f193b741c8e5ed879498e7374bf1744f6a72566745e21012cec8412ea8b8eab9712c740cc71c1c798c1d2df850426f69ce311ff1f537b87ae9a31935525eda54780f15dc73214043ece83f1629e36c55747af1ad8e84fa5e535422b09db9a3145022691e189c9ccbacae70ebea3f6b5f2e09376b6402767c85a2ede43761b4ee5edb436d9f711dfdd84e1ff0dbd35c607ea59d9d5c8adf7fa4bf08634e8cb5027da6bb8a8cd7ba1e87c40be3bd9c5554fae6bf7683d25a57320b63d20c66f0df976dbd688e8c4ccf0dbe755f0d31fd6d1b81c6f1ea0538f73c99cd239efbe92b32b946f07b8b53aba95f05d9e0686e188027061c0c912bf2c27cb83a46cff714e57e4f072bd1df08e23640bb64143ee7c866097f8d360800abbff28a94ef7b035563ce396c9c68ee5c7751c070c364e0fcfb015749b480a8bf4cd9626541e40af879213812612c2e04d688629239f609219a45657693c4cb81c85e156a0c9b377d6123340f009a265cec1a2fde0dfa4676b80dec9b6544712b202cec1467d8f440c2c2a1d13862fab29cb266e9185ed6bbb2c3f3e168bce4720ae079194948d41ed8bf7913fa1d9d8c0dbd39e4f7855f807f6192d3269fa0a98a3d6c27992798b70b59e04dd63918c7ca99054abe15fad0f567d9e993ca49eae695a820552575ee569c4aae328c5db0c3d04d1692ff561064345ed32571d072ea4a2613ce163022287b86b9797223619e556f978deb6711051eab698b43fcac2d62729addc0d699f775b516ed1952af9c5faad43601a4a23007ee2d6fbb10316bbb047e04596a57d5ee871496f85ea460f3f078e6c5a3f629aa6b67e4dbba3ace0a3ebf870a2800a5c7f27a866f6d58b33b1daf49c27a0e8057c169eb9be30d3a42e92676f45be824c12aee3f35a59e1cd23bb39e0ab5a87d43e5894e7aedfa7e74ee6554873db5646eb0e7d2b79f970c4210bd9be2b9294284aa26156ba9773c3422aac0489ba21878bf569c79cc548519221c3c90a706460a2d32e073564d7b6133ef25e14a6e327bc15e33555a299251ca49006225b18d8890b45b49e8c16aab18b6e7ba1e932750e81de679d7b6dff571a4cdf737e06eeec7cbac678b8a58e2142615b71cce7f26187a9c1490407b32c48783bf848a830a41ae89989fe68e50307044c34ac27f360b3679b1e4b76bf590cc131f2e25634c9979b4c98c392fa862cbda18e5c5ac0dd25f3ea6d9fe88691aa4d6399d4bc1da5218b95a8f8ab6683a1916eca043fbdb23137799f8e8e6e240ae4693e1896b942ae548a9e948ba77b2770ac8d98737bdb47d04e5c233da80d84c806e8585e73ab7d4f24ef5415a8ff01392e05b267159736f222a7e88652801cc7b9a3ce955494df7c556d1747255f9492da41ce8dc9a615c9b926f83436190e458f7b3994e425d560f35d3a79753e637c8f7dcfae559e8a76a956927c870e5f6da04a8fbb805253af55e52f3895811825d720e35e305c164ebad8a8f5535e6f1a9023bf6a6f1087361a93741a245b4b98920f539b21958c47b68e8f11bd764b10fba33c607fc70b126263a07216d72eef2d432083a6e373747e345cf6f9362a9bcd52e865062cb026484a21a5794dd4a1171560caec70cb607c324537007b74a36a1ec02033d8fe76196854c4158c47d0535a98eb15b74e347ec308728a32f7b75f06f3ee753cd0598ecc47a563978eba8535eaa012ae9555551a1c3f39c1fadbb66b08cf6f560a820fc4ae281f2bf57fb092f4d2c61568192483e86e65d5d58e86b1a873d06541227228209a98050728216b70185cb0243b893882db43e9a7aaeea366b2dbb13e9c3e707f1be4473d790d676695f1ae7bf1064c6f0a38e8706d37b652cff8e9e1979236a1482dcedb391f7f62da88843221043e472328e7b3fa57e7e06e4eaad1afe822f65002b18382ee5ccd890bd84f26ab467bef4265828e7331c31f119beadf4dca3773ff4a1f1b55b6377bfd9ff5277e740e2c60faa7111547f3ed0325a509582c4185de57a7f0b97b841736df3e69f7f73dfd831967ca615e29c4ffece4c229a3da65ce8e9e7bf12ae10b338290fd80f887cd11cc452ac0b89800c1694e38fbae1d83d7ca0112ad57797354315adceecef61c35fecf618144ed59f84c6f5b9be75de6aa218616bac4eade6dd148b9d692f5df6dcded45b3c3e4a3d679ac91a4eff540f608a86432a749aa119db5e3ea92ceec1ffea5c467bdd5dd3798dd8893e8e3972709025619cada7a880f675faff2a257e18164dbfb1752cfbd5dd8194cfca896298165c0dc4369724b59af981350c4514107c8ff09fa39eed4d76e636a21cdbb3e0c0e5e8f510d087a591135b52f974576432d3e3bf2f46b036c7e140547f96ebabfd0cb242102b3f66aee8560be27657d0900494c33eb8a02dd36b8d8b0e6df3d9931fceeefbc32590050838eec27fa778ae6347102442dd21e300b50faf8fdfabbc1161a6bfe5bc795fbcda8994091ee954edf66f1ba75c4acf7965b31d56db077396bb1e7e9c8286e63e54b34cf5ca11bd8998e40f2c4594c55d7fa8fc2cf9476612d567996307d21caf06e49daf16300bc1f70d70d145e64d92a0b307df6a3d3554d6ccd0c550ba193ff6b818c6f2b0008f17359e8617aa927b9f771203faf28c2a04116f8f048ab0228adad39c8aa3e8c8381b1e60886138face97ae7f3c02d2ef854fab2ef27425b23fcf05bcbeb3d752db854313084ddf7a8833369c17123c92d63c744df22ffaf10188b2f4f4cdb53e5f530279d4419b0858d44a8a8f76a26b78be327bd233bf12709ea04a4cc6b6ccfb20808519f316d293bb4d605e4cee8373202f4e95e28bfef9c42354672526a7a81391239783b64d74c2b04686438411614e7228d34e457151f71d742bc514aedfc96966ea35297d5efa513e8dbbc10e8e16eac2ae14848a20c165f59f1362b57b457f6786b0e1fd492ca47d6aba03774e58f4e09fb480156b9c96ddd13c441fc8202e40a12b30396482778ebc439b5abebcd140ad3848726b2908f6b594e0ac7b307857d68fad4ab05e4362140ad2e4b4606e365e894224dfc033f4c069dafdd7f819d340f6a202592243a27f67a38f20e5b07136f5b503c3ca209736c0a7b99749a9d36a82b2a6926a426c95b5f60c835ab85038469e1e5bce2b6bc2d47345d8f1beed8e678423ca3db106b554890e8a701e5d43f6358d957b8c799d1cf20da1f577df0261717227113e5d1127849bccc076947beab3e3aed964b83c94de9322591b67d6fad860d5a5c642a545aee2d1ee6756723a96d74ae2f431876fce6d8da50628fc51560760c97aa66a258f41a9d958a1e3bb0e27a58eda5f6806a115ac6aadab545aebe2d9c6c6c01cb2dc7e8b36c7b37609563f84f409b0813047fea9220872becbc9cd976d502576d1dc2a0a5612a7a1452e46b35149528ed2cb74b18a221c2281d7b766d28a30f763d7375f6242f4c361807efb711d21426696f77606cf1c37c4c162fe79e444c622297b0849c88995cf248a47c49d1932b6ee79c2c2f3dc77fe97389855c50cb797b0fddaa94c31d5b23f4b7f7d24131aa24d6d53e9a43a3e2c360c3254573355ddbe850299f7ad13f8d0676c8a39ee2a63ec700eb3b53d9abee536a6a2bef272527ede483a09913d8d839cf69c828654ccac0581552b342b0735f49cd5af77bca8e8926173e58320fd4738e574114736cd480900c001766cc72266f7c7ee34dd27a109ad4e932c28d0ce1292df46a0df42f23a48531da38ee14bae23d27e3370434efb85546c5e53c10e8407084e833a65a91d07df741174fbdd7e1e5475d273b1c30fc3db1227d32739889b45ac3ce208f82523d51e2c5a54b6c325ed1fbda4748b61090a838bdc5ebdbf1eb2280ad73028ed8ec40fda49cdf17ce0e158521ea5c99cb8fca79a91e7a4011e6aefa4235b4cdd5344356ed7fb31b1e575592fa2dd89c0a2ce151e07d049d67463d6fff3b2b830451fd89c596d9c807f57aba74fba0a24649e27c849376ab134c5c7f501fd1746b75866f3e74f854029e2e73455a60ec76df9df208b8ce5feccf2b6a2bb743e461924cbd22b91c9e89dcbe231474f26678dd91bda846034f4d55ae97aa1a569c5a3bbb399682ba6b066597a1be9d9a1612cf3faa7cd9b0dc9a3d47c9e382604d7004bb10b2e29fadfa2e374c806a1c7f95b169c4e461bcbe907d3a7ffa6af042f351fc908dd67c522802f0ef18eb57ecfc6f2e95322928e5b561d715a29667ee44382be6eba5ea8577fc926ba742d0d1f345263991610174ec46c5989d8546cd37904089d5327e0787d136d3b13412b536bbdc385990ed35cf6454874c9ce362acb0a8146f78b3f20140c34519c9be23051ad0e0620d50dc3319bfe072c46952cccfa44a62c7afeb9bd596673dcaead2a279175b9324c922a6c17959eef9e6b15a1d952bdbdd74e6eaca13735db8132e0c26fd8eaac1acc66feb76e62760e6746dba93492891a048babd276ae5c4ec2fdd0afc8c1e570a2be342e6d98c3ed004e627ce95e06770227f7b6976a0b2b8e2769e8a1061b15834582c581cc36c0e28909153881e4ee0f8770392a04b3459b1bea67a4f4a68270000"
        ).unwrap();
    let transaction =
        elements::Transaction::consensus_decode(&mut &tx_bytes[..]).expect("decode transaction");

    let spk_input_1 = elements::Script::from(vec![
        0xa9, 0x14, 0x10, 0xc4, 0x65, 0x2c, 0x0d, 0x2d, 0xf7, 0xaf, 0xaa, 0xaf, 0x82, 0x0e, 0x48,
        0x9c, 0xb2, 0x7f, 0xae, 0x60, 0xd4, 0x86, 0x87,
    ]);
    let interpreter = miniscript::Interpreter::from_txdata(
        &spk_input_1,
        &transaction.input[0].script_sig,
        &transaction.input[0].witness.script_witness,
        0,
        0,
    )
    .unwrap();

    let desc_string = interpreter.inferred_descriptor_string();
    println!("Descriptor: {}", desc_string);
    miniscript::Descriptor::<bitcoin::PublicKey>::from_str(&desc_string)
        .expect("this descriptor can be reparsed with sanity checks passing");
    interpreter
        .inferred_descriptor()
        .expect("we can use this method to do the above from_str for us");

    // 1. Example one: learn which keys were used, not bothering
    //    to verify the signatures (trusting that if they're on
    //    the blockchain, standardness would've required they be
    //    either valid or 0-length.
    println!("\nExample one");
    for elem in interpreter.iter_assume_sigs() {
        // Don't bother checking signatures
        match elem.expect("no evaluation error") {
            miniscript::interpreter::SatisfiedConstraint::PublicKey { key_sig } => {
                // Check that the signature is ecdsa sig
                let (key, sig) = key_sig
                    .as_ecdsa()
                    .expect("Expected Ecdsa sig, found schnorr sig");
                println!("Signed with {}: {}, sighash type: {}", key, sig.0, sig.1);
            }
            _ => {}
        }
    }

    // 2. Example two: verify the signatures to ensure that invalid
    //    signatures are not treated as having participated in the script
    let secp = secp256k1_zkp::Secp256k1::new();
    let interpreter = miniscript::Interpreter::from_txdata(
        &spk_input_1,
        &transaction.input[0].script_sig,
        &transaction.input[0].witness.script_witness,
        0,
        0,
    )
    .unwrap();

    // Get the previous confidential amount
    let conf_val: Vec<u8> = elements::hashes::hex::FromHex::from_hex(
        "080e8899a3c271573359a179b27b59af180b36461f959ee00f762d9c2d84192a06",
    )
    .unwrap();

    let amount = confidential::Value::from_commitment(&conf_val).unwrap();
    // We only need to set the amount in prevouts because segwit transactions only need the amount field
    // For taproot transactions, this must contain all the required prevouts
    let mut spent_utxo = elements::TxOut::default();
    spent_utxo.value = amount;
    // Create a spend utxos, since it is a segwit spend we don't really need all prevouts. Fill dummy data for this example instead
    let utxos = [spent_utxo, elements::TxOut::default()];
    let prevouts = elements::sighash::Prevouts::All(&utxos);
    // segwit spends don't require genesis hash
    let genesis_hash = elements::BlockHash::default();

    println!("\nExample two");
    for elem in interpreter.iter(&secp, &transaction, 0, &prevouts, genesis_hash) {
        match elem.expect("no evaluation error") {
            miniscript::interpreter::SatisfiedConstraint::PublicKey { key_sig } => {
                let (key, sig) = key_sig.as_ecdsa().unwrap();
                println!("Signed with key {}: sig {} hash_ty: {}", key, sig.0, sig.1);
            }
            _ => {}
        }
    }

    // 3. Example three: same, but with the wrong signature hash, to demonstrate
    //    what happens given an apparently invalid script
    let secp = secp256k1_zkp::Secp256k1::new();
    let message = secp256k1_zkp::Message::from_slice(&[0x01; 32][..]).expect("32-byte hash");
    let interpreter = miniscript::Interpreter::from_txdata(
        &spk_input_1,
        &transaction.input[0].script_sig,
        &transaction.input[0].witness.script_witness,
        0,
        0,
    )
    .unwrap();

    let iter = interpreter.iter_custom(Box::new(|key_sig: &KeySigPair| {
        let (pk, ecdsa_sig) = key_sig.as_ecdsa().expect("Ecdsa Sig");
        ecdsa_sig.1 == elements::EcdsaSigHashType::All
            && secp.verify_ecdsa(&message, &ecdsa_sig.0, &pk.inner).is_ok()
    }));
    println!("\nExample three");
    for elem in iter {
        let error = elem.expect_err("evaluation error");
        println!("Evaluation error: {}", error);
    }
}
