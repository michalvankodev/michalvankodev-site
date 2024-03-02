---
layout: blog
title: I built my 3rd custom keyboard
segments:
  - blog
published: true
date: 2023-08-29T19:34:17.071Z
tags:
  - News
  - Keyboards
---
## My keyboard journey

When I was young and still had time to play games on a regular basis I got myself an 80% mechanical keyboard from [Cooler Master](https://www.coolermaster.com/catalog/peripheral/keyboards/quick-fire-rapid-i/). I didn't want to waste any space with a useless num-pad. For a very long time, I was very satisfied with the keyboard. It still serves me today. It has [cherry brown](https://www.cherrymx.de/en/cherry-mx/mx-original/mx-brown.html) switches. I got very used to them, but I always wanted to try something different.

A few years later I found the [r/MechanicalKeyboards](https://www.reddit.com/r/MechanicalKeyboards/) subreddit. I was amazed by the collection of custom-made keyboards and the variety of switches and other customization that is available.
I decided to **make my own split orthodox Redox keyboard** with **very light switches** so I could type fast. At least that's what I was thinking, but after a few days, I realized that those switches were a mistake. There are so many articles on the internet about how to pick switches, but I guess **it's just finding what is good for each person by experience**.

![My custom redox keyboard](/images/uploads/img_20200301_121954.jpg "My custom redox keyboard")

At the same time, **I've switched to the [Colemak layout](https://colemak.com/)**. This one was perhaps the **best choice that I could make** at that time. My wrists were hurting and the ease of typing words with much less friction really helps out today. The time of struggle was definitely worth it.
There is this new layout on the block called [Workman](https://workmanlayout.org/), where the author is going through some tests calculating various metrics on written novels. It can be easily deductible that no matter what layout you choose, it will save you many many meters of travel in comparison to QWERTY.

While I was using my Redox for a few months I got to know what is the potential of [QMK](https://docs.qmk.fm/#/). I figured that I didn't need that many keys on the keyboard. I was searching for a minimalistic keyboard and setup and I found Redox's smaller cousin [Minidox](https://github.com/That-Canadian/MiniDox_PCB).
I've opted for a 36-key layout with 2 keys dedicated to switching between 4 different layouts. It was a very nice experience. I was amazed that **I could fit a full keyboard functionality with media keys onto a 36-key keyboard**. I've opted for brown switches back again. I didn't want to risk making high noise while writing on a keyboard and I was missing the tactile feel when you press the key.

![My Minidox keyboard](/images/uploads/20210420_155540_03.jpg "My Minidox keyboard")



The only downside of these keyboards was that they had very fragile USB ports on the ProMicros. I was very hesitant whether I should take them with me to the office or on the road. It happened multiple times that I had to solder out the ProMicros and substitute them with a new part. The experience of desoldering a microcontroller from a PCB is horrible.

With the Minidox I was able to survive with only one side having a functional USB port for a longer time, thanks to the [`EE_HANDS` functionality](https://docs.qmk.fm/#/feature_split_keyboard?id=handedness-by-eeprom).
But, I had in my mind, that the second side would fail in the near future so I kept searching for another custom build.

> In the end, I was able to succesfully replace the ProMicro microcontroller so Minidox can serve me when I need a keyboard that doesn't make too much noise.

## Picking the keyboard shield

So **I mastered split 36-key ortholinear keyboard layout**. How could I improve from that?
In commercial space, the Kinesis 360 was trending and coming out. I was waiting and thinking about whether I should buy it. The reason why is the Concave keywells. In the DIY space, there were Dactyls.
You can find generators for Dactyls on the internet, and I was playing with the source code of a [Dactyl Manuform](https://github.com/abstracthat/dactyl-manuform) myself. I couldn't find settings that would be appropriate and I haven't been able to test the generated models myself, because I didn't have a 3D printer at that time.
Therefore I kept waiting and once I thought that it was time to **create my own design** for the keyboard.

I started a [side project with Rust and OpenSCAD](https://github.com/michalvankodev/rusteaks). As a first model, I've designed a parametrized headphone holder as a table attachment. Then I started to work on the custom keyboard and it didn't go according to my imagination. I **spent too much time** with the math. Rotating the individual pieces inside another piece and so on was really hard. I like algebra, don't get me wrong. But, after spending too much time just around a row of holes for the switches. I couldn't even estimate how long it would take me to design the edges and the thumb cluster, holes for screws, and other parts that are much more complicated.

**I wanted to build the actual keyboard**, so I got to find something that I was aiming for anyway. I was mostly amazed by the work of [Bastard Keyboards](https://bastardkb.com/). I've opted for the [Skeletyl](https://github.com/Bastardkb/Skeletyl/). It has the same 36-key layout that I am comfortable with and it has a nice tent and is **easy to 3D print**.

## Shopping list

The most important thing is to order all the necessary parts for the build. Most of them will **take a full month** to arrive when you order from all around the internet.
I already had the spare diodes from the other 2 builds but I will include them in the shopping list so you can include them in your order.

I've tested some clicky switches, and they did feel amazing to me so I've opted for noisy switches.

Another customization is that while the shield is designed for a wired connection between the splits, I've decided that I want to try [ZMK](https://zmk.dev/) and **build a wireless keyboard**.

> Tip: If you plan on using your keyboard with a desktop computer, you might want to consider a wired connection. BIOS boot up check will not pass without a physical connection to a keyboard. I have to have 2 keyboards on my desk now :( 

| Item                                                  | Quantity    | Price   | Link                                                                                                               |
| ----------------------------------------------------- | ----------- | ------- | ------------------------------------------------------------------------------------------------------------------ |
| DSA Blank Keycaps                                     | 4           | 14 EUR  | [🛒](https://www.aliexpress.com/item/1005002587285218.html?spm=a2g0o.order_list.0.0.3e541802H3XiA6)                |
| Brass Hot Melt Inset Nuts M4 X D6.0 X L5.0            | 6 of 50pcs  | 3 EUR   | [🛒](https://www.aliexpress.com/item/4000232925592.html?spm=a2g0o.order_list.order_list_main.43.6e0d1802Xk8ijl)    |
| M4 * 8 Screws                                         | 6 of 100pcs | 3 ELR   | [🛒](https://www.aliexpress.com/item/32896139810.html?spm=a2g0o.order_list.order_list_main.38.6e0d1802Xk8ijl)      |
| 3.7V 500mAh Li-Po Rechargeable Battery                | 2           | 6 EUR   | [🛒](https://www.aliexpress.com/item/1005003262232565.html?spm=a2g0o.order_list.order_list_main.5.6e0d1802Xk8ijl)  |
| 40PIN 20CM Female To Female Jumper Dupont Wire Cables | 1-3         | 5 EUR   | [🛒](https://www.aliexpress.com/item/33041631448.html?spm=a2g0o.order_list.0.0.3e541802H3XiA6)                     |
| USB-C Male-to-Female Extension Cable                  | 2           | 6.5 EUR | [🛒](https://www.aliexpress.com/item/1005003238859317.html?spm=a2g0o.order_list.order_list_main.63.6e0d1802Xk8ijl) |
| Mini ON-OFF switches                                  | 2           | 2 EUR   | [🛒](https://www.aliexpress.com/item/32918008270.html?spm=a2g0o.order_list.order_list_main.16.6e0d1802Xk8ijl)      |
| KAILH BOX JADE Switches                               | 40          | 24 EUR  | [🛒](https://www.eloquentclicks.com/product/novelkeys-x-kailh-box-jade-switch/)                                    |
| nice!nano V2 Wireless Controller                      | 2           | 52 EUR  | [🛒](https://42keebs.eu/shop/parts/controllers/nice-nano-v2-wireless-controller/)                                  |

I attempted to print the keyboard shield myself, but I've been only successful in printing the tents. I had to ask my friend to print me the upper body of the Skeletyl. The shield can be easily printed in a PLA so it is really cheap. I can sum up the costs to 10 EUR for the parts.

![3D printed shield of the Skeletyl](/images/uploads/20230223_083626-1-.jpg "3D printed shield of the Skeletyl")



## Assembly

The assembly was done on [my twitch stream](https://twitch.tv/michalvankodev) in the span of a single weekend. It took perhaps around **7 hours in total**.
I've managed to solder everything together. The hardest part was to take off the isolant of the cables. I hurt my fingers a bit. 

![Wiring of switches and diodes](/images/uploads/20230226_130037.jpg "Wiring of switches and diodes")



At first, it seemed that the keyboard would not work. It was not responding at all and I was tired already to debug it that night.
I almost didn't sleep as I was browsing through all the possible mistakes I could've made. I stumbled on a *reddit* thread where someone mentioned that the issue might also be a **different way of wiring the grid**.
That was my assumption so I could pick up on the issue of burning the firmware the next day.
[This little change](https://github.com/michalvankodev/zmk-config/commit/3a77216f9f371ca331d29d952aee2805c52b7ef0) in the *ZMK* configuration was enough to make the keyboard responsive. But it was not all that I had to change. With this setting the keyboard acted as if all the **keys were pressed simultaneously** at all times. Imagine what happens to your computer when you have the whole keyboard pressed. Random keyboard shortcuts have been triggered and my whole desktop was blown away with a tornado. I had to [change more settings](https://github.com/michalvankodev/zmk-config/commit/eceeb9e28dcbad0bff28c61ca032c3f991d5d4e6) to make the keyboard act like a proper keyboard.
These changes basically tell the keyboard CPU how to read signals from the switches and diodes and where the signal is coming from. 

I made a ["short" video](https://www.youtube.com/watch?v=EaNyRKruxJE) of the whole assembly from the stream footage.

<div class="video-embed">
<iframe height="100%" width="100%" src="https://www.youtube.com/embed/EaNyRKruxJE" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>
</div>

## Keyboard layout

With the new keyboard, I got a chance to explore more options regarding the layout. I was interested in **home row mods** and I've found out about the [Miryoku](https://github.com/manna-harbour/miryoku/) layout. I've applied little changes to it but the layers are what are so special about this layout. With this layout, I am **able to press all the keyboard shortcuts** I use in the desktop environment with ease. Before I had to leverage sticky keys to be able to press multiple keys from different layers. Now **I don't have a single sticky key mapped** as all of the functionality can be easily achieved with Miryoku. Go ahead and read the [Miryoku reference manual](https://github.com/manna-harbour/miryoku/tree/master/docs/reference). I might be looking into changing the main layout to Colemak DH or Workman, but **I'm not going away** from the rest of the Miryoku.

## Switching switches

After a few months with the Jade BOX switches, I noticed that the **click was not very comfortable** for my thinking. My **brain was too focused** on the click sound and Jades performed this sound twice - once on key down and then again when the key was released. The loud sound is also annoying to anyone who is sitting next to you.
I've attempted to retry and revive the printer that I have at home and after a few days of failed attempts, I've decided to once again ask my friend to reprint the shields so I can install different switches and have 2 separate shields.
While I was working on my [presentation about keyboard ergonomics](https://michalvanko.dev/broadcasts/2023-04-27-keyboards-ergonomics-of-21st-century) I did some research on the tactile switches and stumbled upon [Glorious Panda Tactile Switches](https://www.gloriousgaming.com/products/glorious-panda-mechanical-switches?variant=37691905933487). They are being sold in exactly 36-piece bundles.
The assembly of just the switches was simple and I got it working right away. I just pulled the nice!nano board out and put it into the new shield. Two switches have been struggling as the space was tight so I had to grind some of the plastics off to enlarge the space a little bit. After that, I figured that **I am truly a tactile kind of guy**.
I got used to the tactile switches immediately and I was able to **type faster** right away.

## Journey continues

I'd say that this Skeletyl is a **magical keyboard with a lore**. It is constructed from 2 different types of filament printed on 2 different printers. It has 2 different upper bodies, that can be switched. Just like a skeleton from a fiction story. It is a magnificent piece that serves me very well.

But **this is not where the journey ends**. I already can feel some aspects that I'd change about this keyboard. I'd like to go another level up. Just like the brother of Skeletyl, [Charydbis](https://github.com/Bastardkb/Charybdis/), has a trackball. I'd like to see if I am able to **construct an ultimate tool that would replace my mouse**. I'd be able to not move my hands at all which could turn very beneficial for my wrists.
I also would like to **position the thumb keys lower**. I'd like to have the most relaxing hand position when I'm not typing. On the Skeletyl with the tents, the thumb cluster is high. So it seems like I will be working on my own design in the end. I'm very excited to play with the track-ball or a track-pad and see how to integrate one. I'd also like the **next keyboard to be wired.** Every computer with bios will not start up without a keyboard being plugged in. That's how it has been for almost a century.
