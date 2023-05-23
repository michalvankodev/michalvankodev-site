---
layout: blog
title: I built my 3rd custom keyboard
segments:
  - blog
published: true
date: 2023-05-23T19:34:17.071Z
tags:
  - News
  - Keyboards
---
## My keyboard journey

When I was young and still got time to play games on a regular basis I got myself a 80% mechanical keyboard from [Cooler Master]. I didn't want to waste any space with a useless num pad. For a very long time I was very satisfied with the keyboard. It still serves me today. It has [cherry brown] switches. I got very used to them, but I always wanted to try something different.

Few years later I've found out about [r/MechanicalKeyboards](https://www.reddit.com/r/MechanicalKeyboards/) subreddit. I was amazed by the collection of custom made keyboards and the variety of switches and other customization that is available.
I decided to make my own split orthodox Redox keyboard with very light switches so I can type fast. At least that's what I was thinking, but after few days I've realized that those switches were a mistake. There are so many articles on the internet about how to pick switches, but I guess it's just finding what is good for each person by the experience.

At the same time I've switched to [Colemak layout](https://colemak.com/). This one was perhaps the best choice that I've could make at that time. My wrists were hurting and the ease of typing words with much less friction really helps out today. The time of struggles was definetely worth it.
There is this new layout on the block called [Workman](https://workmanlayout.org/), where the author is going through some tests calculating various metrics on written novels. It can be easily deductible that no mather what layout you choose, it will save you many many metres of travel in comparison to QWERTY.

While I was using my Redox for few months I got to know what is the potential of [QMK](https://docs.qmk.fm/#/). I figured that I don't need that many keys on the keyboard. I was searching for a very minimalstic keyboard and setup and I've found Redox's smaller cousin [Minidox](https://github.com/That-Canadian/MiniDox_PCB).
I've opted for a 36 key layout with 2 keys dedicated to switching between 4 different layouts. It was a very nice experience. I was amazed that I could fit a full keyboard functionality with media keys onto a 36 key keyboard. I've opted for brown switches back again. I didn't want to risk to have a high noise while writting on a keyboard and I was missing the tactile feel when you press the key.

The only downsides of these keyboards was that they had very fragile USB ports on the ProMicros. I was very hesitant whether I should take them with me to the office or on the road. It happened multiple times that I had to solder out the ProMicros and substitue them with a new part. The experience of desoldering a microcontroller from a PCB is horrible.

With the minidox I was able to survive with only one side having a functional USB port for a longer time, thanks to the [`EE_HANDS` functionality](https://docs.qmk.fm/#/feature_split_keyboard?id=handedness-by-eeprom).
But, I had in my mind, that the second side will fail also in the near future so I kept searching for another custom build.

> In the end, I was able to succesfully replace the ProMicro microcontroller so Minidox can serve me when I need a keyboard that doesn't make too much noise.

## Picking the keyboard shield

So I mastered split 36 ortholinear keyboard layout. How could I improve from that?
In commercial space the Kinesis 360 was trending and coming out. I was waiting and thinking whether I should by it. The reason why is the Concave keywells. In DIY space there where Dactyls.
You can find generators for Dactyls on the internet, and I was playing with the source code of an [Dactyl Manuform](https://github.com/abstracthat/dactyl-manuform) myself. I couldn't find settings that would be appropriate and I haven't been able to test the generated models myself, becouse I didn't had a 3D printer at that time.
Therefore I kept waiting and once I thought that, it is time to create my own design for the keyboard.

I started a [side project with Rust and OpenSCAD](https://github.com/michalvankodev/rusteaks). As a first model I've designed a parametrized headphones holder as a table attachment. Then I started to work on the custom keyboard and it didn't go according to my imagination. I spent too much time with the math. Rotating the individual pieces inside another piece and so on was really hard. I like algebra, don't get me wrong. But, after spending too much time just around a row of holes for the switches. I couldn't even estimate how long it would took me to design the edges and the thumbcluster, holes for screws and other parts that are much more complicated.

I wanted to build the actual keyboard, so I got to find something that I was aiming for anyways. I was mostly amazed by the work of [Bastard Keyboards](https://bastardkb.com/). I've opted for the [Skeletyl](https://github.com/Bastardkb/Skeletyl/). It has the same 36 key layout that I am comfortable with and has a nice tent and is easy to 3D print.

## Shopping list

The most important thing is to order all the necessary parts for the build. Most of them will take a full month to arrive when you order from all around the internet.
I already had the spare diodes from the other 2 builds but I will include them in the shopping list so you can include them in your order.

I've tested some clicky switches, and they did feel amazing to me so I've opted for a noisy switches which I regret now. (I will be probably doing another build soon).

Other customization is that while the shield is designed for a wired connection between the splits, I've decided that I want to try [ZMK](https://zmk.dev/) and build a wireless keyboard.

> Tip: If you plan on using your keyboard with a desktop computer, you might want to consider a wired connection. BIOS boot up check will not pass without a physical connection to a keyboard. I have to have 2 keyboards on my desk now :( 

| Item                                                               | Quantity | Price   | Link |
|--------------------------------------------------------------------|:--------:|--------:|--------------------------------------------------------------------------------------------------------------|
| DSA Blank Keycaps                                                  | 4        | 14 EUR  | [🛒](https://www.aliexpress.com/item/1005002587285218.html?spm=a2g0o.order_list.0.0.3e541802H3XiA6) | 
| Brass Hot Melt Inset Nuts M4 X D6.0 X L5.0  (50pcs, only 6 needed) | 1        | 3 EUR   | [🛒](https://www.aliexpress.com/item/4000232925592.html?spm=a2g0o.order_list.order_list_main.43.6e0d1802Xk8ijl)    |
| M4 * 8 Screws (100pcs, only 6 needed)                              | 1        | 3 ELR   | [🛒](https://www.aliexpress.com/item/32896139810.html?spm=a2g0o.order_list.order_list_main.38.6e0d1802Xk8ijl)      |
| 3.7V 500mAh Li-Po Rechargeable Battery                             | 2        | 6 EUR   | [🛒](https://www.aliexpress.com/item/1005003262232565.html?spm=a2g0o.order_list.order_list_main.5.6e0d1802Xk8ijl)  |
| 40PIN 20CM Female To Female Jumper Dupont Wire Cables              | 1-3      | 5 EUR   | [🛒](https://www.aliexpress.com/item/33041631448.html?spm=a2g0o.order_list.0.0.3e541802H3XiA6)                     |
| USB-C Male to Female Extension Cable                               | 2        | 6.5 EUR | [🛒](https://www.aliexpress.com/item/1005003238859317.html?spm=a2g0o.order_list.order_list_main.63.6e0d1802Xk8ijl) |
| Mini ON-OFF switches                                               | 2        | 2 EUR   | [🛒](https://www.aliexpress.com/item/32918008270.html?spm=a2g0o.order_list.order_list_main.16.6e0d1802Xk8ijl)      |
| KAILH BOX JADE Switches                                            | 40       | 24 EUR  | [🛒](https://www.eloquentclicks.com/product/novelkeys-x-kailh-box-jade-switch/)                                    |
| nice!nano V2 Wireless Controller                                   | 2        | 52 EUR  | [🛒](https://42keebs.eu/shop/parts/controllers/nice-nano-v2-wireless-controller/)                                  |


