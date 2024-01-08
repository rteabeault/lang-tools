// <transcript>
//   <text start="3.34" dur="4.8">Es ist der 21.04.2016. Nachts in der Nähe von Höxter.</text>
//   <text start="8.7" dur="4.44">Der Rettungswagen bringt eine 41 Jahre alte Frau ins Krankenhaus.</text>
//   <text start="13.18" dur="3.08">Sie ist nicht ansprechbar, unterkühlt, verwahrlost</text>
//   <text start="16.26" dur="2.28">und hat eine Schädelverletzung.</text>
//   <text start="22.3" dur="3.72">7 h später stirbt Susanne F. an ihren Verletzungen.</text>
//   <text start="26.02" dur="2.8">Ihr Körper ist übersät mit Hämatomen.</text>
//   <text start="32.34" dur="3.12">Die Spur derjenigen, die ihr das angetan haben sollen,</text>
//   <text start="35.46" dur="2.72">führt in dieses Haus in Höxter-Bosseborn.</text>
//   <text start="38.74" dur="2.52">In diesem Haus aber soll nicht nur Susanne F.</text>
//   <text start="41.26" dur="1.8">zu Tode gequält worden sein.</text>
//   <text start="47.34" dur="2.68">Ich bin zu Hause, stehe am Bügelbrett und sehe...</text>
//   <text start="50.02" dur="2.72">Ich weiß nicht, welche Sendung das war, BRISANT.</text>
// ...
// </transcript>

#[derive(Debug)]
struct Text {
    pub text: String,
    pub start: f32,
    pub duration: f32
}

#[derive(Debug)]
struct Transcript {
    pub texts: Vec<Text>
}

#[derive(Debug)]
struct Sentence {
    // A sentence is made up of one or more transcript Texts
    texts: Vec<Text>
}

/// Combine the different texts into sentences.
impl From<Vec<Text>> for Vec<Sentence> {
    fn from(value: Vec<Text>) -> Self {
        todo!()
    }
}