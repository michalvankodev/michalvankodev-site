---
layout: blog
title: Custom Redox keyboard assembly
published: true
date: 2020-04-10T15:16:54.820Z
thumbnail: /images/uploads/img_20200301_171735.jpg
tags:
  - Keyboards
  - Tutorial
notes: add some stuff about redox keyboard itself
---

In this blog post I am going to take you out on a little journey of making my own keyboard. Perhaps help you if you've decided to make yourself your own.

![Preview of Redox keyboard with RGB lights turned on in a dark](/images/uploads/img_20200301_171735.jpg 'Preview of Redox keyboard with RGB lights turned on in a dark')

But first, I want to tell you:

## Why I've decided to make my own keyboard

I've been feeling pain in a wrist of my right hand for 2 years now. I've started to feel it when I started to be more physically active. I've been working out and played a lot of squash. It got to the point that I had to visit an orthopedist. I was scared that I'd have the infamous **carpal tunnel syndrome**. At this stage he said it was just strained. The pain eased out after some therapy but it still comes back and it got to the point that I had to change my work habits.

One night I was checking out the state of programming streams on _twitch_. I've found a great streamer [JMS WRN](https://www.twitch.tv/jmswrnr) and as I was chatting with him about my wrist problem he gave me a recommendation for my new mouse: [Logitech MX Vertical](https://www.logitech.com/en-us/product/mx-vertical-ergonomic-mouse?crid=7) and then he showed me that he has a _Discord channel_ just for keyboard stuff. And that was **the moment** for me. I've discovered a whole new world of possibilities around keyboards.

I've started to do a research around custom keyboards, their parts, _QMK software_ and its possibilities. I was 100% sure that, if I am going to make my own keyboard it would be **split and ergonomic**. It just looks so cool to be able to have one part of the keyboard away from the other.

### Some split keyboards that caught my attention

- [Iris Keyboard](https://keeb.io/collections/frontpage/products/iris-keyboard-split-ergonomic-keyboard?variant=29480467267678)
- [Quefrency - 60%/65%](https://keeb.io/collections/frontpage/products/quefrency-60-65-split-staggered-keyboard?variant=16032981385310)
- [ErgoDox](https://www.ergodox.io/)

I got to the point when I realized, that if I want to make a keyboard, I have to make a complete shopping list. My first go to was the _Iris Keyboard_ from _Keeb.io_. But then I've found the shipping cost and that changed my decision. I was looking into other options that were accessible in Europe and then I've found [FalbaTech](https://falba.tech/), who is based in Poland which made me so excited. There are really not that many shops which are based in Europe.

I took a look at his selection of custom keyboards and decided on the **Redox keyboard**. It is a split orthodox keyboard heavily inspired by **Ergodox**. There are two options for this keyboard: Classic wired and wireless. I've decided for a wired one as I didn't want to complicate my first build.

If you want to take a closer look into Redox keyboard and its details, I recommend reading designer's [Hackaday.io page](https://hackaday.io/project/160610-redox-keyboard)

## Let's make a shopping list

You might think that making a custom mechanical keyboard can be cheap but the opposite is quite true. The electronics are the cheapest part and they are not that hard to get. You might find many of them in the nearest radio amateur shop. Parts that you have to reconsider and are most important are the PCB, case, switches and keycaps.

For my build, as it is my first one, I had to also buy soldering iron and various tools.

I decided that I want my custom case and my friend was so kind that he asked his brother to **3D print** it for me.

- [3D printed case](https://www.thingiverse.com/thing:2886662) - 32€

  - I've paid only for materials but the overall cost is higher

- [REDOX PCB Electrical Boards](https://falba.tech/product/redox-pcb-electrical-boards-set-of-2/) - 20€
- [70pcs Pack 3Pin Gateron White Switch](https://www.banggood.com/70PCS-Pack-3Pin-Gateron-White-Switch-Keyboard-Switch-for-Mechanical-Gaming-Keyboard-p-1446975.html?rmmds=category&cur_warehouse=CN) - ~18€

  - I have to say that these switches are VERY light and they are difficult to get used to.

- [DSA Profile PBT Blank Keycaps](https://www.banggood.com/104-Key-DSA-Profile-PBT-Blank-Keycaps-Key-Caps-Set-for-Mechanical-Keyboard-p-1287125.html?rmmds=myorder&ID=520016&cur_warehouse=CN) - ~23€
- [2x Pro Micro ATmega32U4](https://www.banggood.com/Pro-Micro-5V-16M-Mini-Leonardo-Microcontroller-Development-Board-p-1077675.html?akmClientCountry=SK&) - ~8€
- [RGB LED Strip](https://www.banggood.com/1M-Non-Waterproof-WS2812-WS2812B-RGB-30-LED-Strip-Light-Individually-Addressable-5V-p-1145875.html?rmmds=search&ID=233&cur_warehouse=CN) - ~6€
- [Momentary switches](https://www.aliexpress.com/item/32912263133.html?spm=a2g0o.productlist.0.0.9d673fa24q3hHR&algo_pvid=49d8606d-c49b-4211-bbc1-cd7667735e68&algo_expid=49d8606d-c49b-4211-bbc1-cd7667735e68-0&btsid=89610db5-e204-4d29-b6b9-8f2136fdeffd&ws_ab_test=searchweb0_0,searchweb201602_3,searchweb201603_55) - ~2€
- [3.5MM TRRS Sockets](https://www.aliexpress.com/item/32368285821.html?spm=a2g0s.9042311.0.0.3dea4c4dDeY5X1) ~2€
- [3.5MM TRRS Jacks](https://www.aliexpress.com/item/4000106882350.html?spm=a2g0o.productlist.0.0.40a839853cDhpo&algo_pvid=507b146d-4ff4-4220-8f28-a5351fc63eef&algo_expid=507b146d-4ff4-4220-8f28-a5351fc63eef-9&btsid=0ab6fa8115849986217431938e8598&ws_ab_test=searchweb0_0,searchweb201602_,searchweb201603_) ~2€
- [1N4148 Switching Diodes](https://www.banggood.com/100pcs-1N4148-Switching-Diode-Kit-DIY-Electronic-Component-Set-Straight-Pin-DO-35-p-1182125.html?rmmds=myorder&cur_warehouse=CN) - ~2€

You will also need some cables so make sure that you will buy everything you need.

So this makes a total of ~115 € for only components of the keyboard itself and these are very cheap compared to the commercial market where you could pay this price for keycaps only.

I also had to buy some tools, so I could make this project a reality. There is a nice sum up of tools you'll need at [keeb.io](https://docs.keeb.io/soldering-tools/).

- [60W Soldering Station](https://www.banggood.com/MUSTOOL-SD1-SD2-LCD-60W-Soldering-Station-Professional-PID-Soldering-Iron-Station-4-in-1-Tool-Kit-Adjustable-Temperature-200-480C-with-Solder-Wire-Holder-p-1590175.html?rmmds=myorder&ID=471846280495&cur_warehouse=CN) - ~25€
- [Anti-static Tweezers](https://www.banggood.com/ESD-10-15-Safe-Anti-static-Tweezer-Maintenance-Repair-Nippers-Forceps-p-1102662.html?rmmds=myorder&ID=514829&cur_warehouse=CN) - ~1€
- [Keycap Puller](https://www.banggood.com/Universal-Keyboard-Key-Cap-Puller-for-Mechanical-Keyboard-p-1211430.html?rmmds=myorder&cur_warehouse=CN) - ~4€
- [Solder sucker](https://www.banggood.com/Antistatic-Vacuum-Desoldering-Pump-Irons-Sucker-Removal-Remover-Tool-p-911889.html?rmmds=myorder&ID=6284394&cur_warehouse=CN) - ~3€
- [Flush Cutters](https://www.banggood.com/Pliers-Nipper-H-Practical-Electrical-Wire-Cable-Cutter-Cutting-Side-Snips-Flush-Pliers-Mini-Pliers-p-1371806.html?rmmds=myorder&ID=3160&cur_warehouse=CN) - ~3€
- [Solder](https://www.banggood.com/6337-0_8mm-Tin-Lead-Rosin-Core-Flux-Solder-Soldering-Welding-Iron-Wire-Reel-p-992684.html?rmmds=myorder&ID=6284394&cur_warehouse=CN) - ~3€
- [Helping Hands](https://www.banggood.com/Hand-Soldering-Iron-Stand-Helping-Clamp-Magnifying-Tool-Auxiliary-Clip-Magnifier-Station-Holder-p-1017105.html?rmmds=myorder&cur_warehouse=CN) - ~10€

For a total of ~49€ for tools needed.

## Assembly time

So I waited a few weeks for the goods to arrive home and after everything that was needed for the assembly came I've scheduled the assembly to be done over the weekend. First weekend had came by and I was not feeling it, so I moved it to the next one when I was sure I wouldn't be interrupted by anyone. It fit very well because I was soldering the keyboard over the night that was on a day of parliamentary elections. I knew that I was going to be up very late anyway.

### 1. Prepare your workplace

Find a suitable room which has enough light and can be ventilated. One office desk should be enough for soldering.

So this is how the keyboard looks like when you pack it up into a shoe-box:

![Keyboard parts stored in a shoe-box next to soldering station](/images/uploads/2020-03-23_20-24-13_625.jpg 'Keyboard parts stored in a shoe-box next to soldering station')

### 2. Solder switching diodes

What's nice about keyboard assembly is that, when you are going to solder for the first time or like me the second time after 12 years, it starts from the simplest tasks to the hardest. In the end it is simple all the way because after first 70 soldered joints you will get into it and you will learn how the solder behaves.

PCBs of Redox are the same for both sides. That means you will put diodes on the opposite side of the PCB that it is going to be used for second hand.

![PCB with soldered switching diodes](/images/uploads/2020-03-23_20-24-08_235.jpg 'PCB with soldered switching diodes')

Remember that the **direction of the diodes matters** and that means the black part of the diode should be placed at the square on the PCB!!!

When you are done with soldering be careful while cutting the pins. They are going to fly across whole room. Also you should wear protective glasses if you are not able to point them to be shooting away from you. I was able to find some pins after 2 weeks in the corners of the room.

### 3. Solder momentary switches

Momentary switches are used to send a reset signal to the Pro Micro. They are going to be essential for installing firmware into the Pro Micro.

### 4. Solder TRRS sockets

They will provide a way for connecting two sides together so you would only use one USB connection with computer.

Also, if you decide to install RGB backlights, the coordinating signal is going to be wired through the socket pin as well.

### 5. Solder the head pins for the Pro Micro

By head pins I mean the small pins that are supposed to hold the Pro Micro above the PCB.

Don't solder the Pro Micro yet! You won't be able to put the switches onto the board.

![Head pins of Pro Micro soldered into the PCB](/images/uploads/2020-03-23_20-24-06_393.jpg 'Head pins of Pro Micro soldered into the PCB')

At this point, you would notice that the USB port is going to be very low and not very tight on the left side. Please be aware of this. If you want to, you can also just wire some leftover pins from the diodes and make them shorter but I was doing OK with the head pins of the Pro Micro.

### 6. Mount the switches

Try to put at least few switches on the case and try mounting them on the board. The PCB has holes in it to mount the switches so it should fit together. Mine didn't so I had to make a little hack on a few switches so they wouldn't bend my board by cutting their bottoms.

![Cutting switch bottoms to be able to put them in the case](/images/uploads/2020-03-23_20-24-15_387.jpg 'Cutting switch bottoms to be able to put them in the case')

### 7. Solder the switches

All of the switches should have their feet on the other side of the board. Try to push them closer to the board so it will be easier for the solder to make a joint.

### 8. Solder the Pro Micro

Almost done with the solder. Now it's time to wire the brains of the keyboard. These joints are going to be the best. There is not that much room so you'll have to bring your A game here.

To be honest, I had to revisit and try to fix both of my Pro Micros. One full column of my keyboard was not working because of one bad contact. I was trying to unmount whole Pro Micro out but wasn't able to suck the solder out so I desperately just tried it out again and it worked. I just needed to apply more heat to it. Few days later, my Enter key was not working and it was the same problem, but on the other side.

Before you proceed to the next step:

The USB ports on the Pro Micro are infamous to break when you manipulate a lot with them. I've decided that I want to have my right hand side as a master and the USB port broke after 3 days. I was not able to burn a new firmware on it but I had to rewire the RGB backlight and I use my left hand side as a master. On the left hand side the USB port is sitting between the board and Pro Micro so it should be harder to break. Be careful with them anyway!

### 9. Wire RGB backlight

Find some small cables that are flexible and wire them according to this: https://github.com/mattdibi/redox-keyboard/tree/master/redox#rgb-underglow

## Burning the firmware

I'll recommend to get to know QMK software. You can do so here: https://qmk.fm/

Check it's abilities and _Getting Started_ section

But to sum up what I did it was pretty simple:

1. `git clone https://github.com/qmk/qmk_firmware`
2. Set the number of RGB diodes in `/keyboards/redox/keymaps/default/config.h`
3. Made my own keyboard layout over: https://config.qmk.fm/#/redox/rev1/LAYOUT
4. Burn it `make redox/rev1:default:avrdude`
   - Of course I've struggled a little bit with this part. I've ended up with running this command with `sudo` as I wasn't able to set some stuff described here: https://docs.qmk.fm/#/faq_build?id=can39t-program-on-linux

In case you want to check out my configuration you can [download it here](/files/redox_keyboard_layout.json 'Keyboard layout mappings for QMK configuration tool') and upload it into the configurator. I've added some multimedia keys in a third layout and much more.

And that's it. You have built yourself a keyboard.

## Feedback

Well, I just made a big change in my experience with writing. I was learning to write with 10 fingers and if that wasn't the biggest change I've made. I've decided that if I am going to learn to write properly I'll do it even better. I've learned to use the **Colemak keyboard layout**. To enhance it to the best experience possible I've built myself an **orthodox split keyboard** and I must say: **_WORTH IT!_**

So what are the little things that come with switching to orthodox keyboard, you may ask. Well it depends on your habits. Mine were certainly very bad as I didn't know how to write with 10 fingers. Only 8 in my case. Also I was used to type H and Y keys with my left hand. But that was not the biggest problem. Most errors I've made was by trying to hit `C` key. I was used to hitting it with my pointer finger which I think is not only my issue. With the classic keyboard design, hitting the `C` key with middle finger seems like a torture. After one month I am now able to properly hit any key with the correct finger. I just still do many mistakes because of the layout change.

I've chosen very light switches because I wanted them to be less noisy as I work in a shared office and also my ol' _Cherry browns_ seem a little too hard to me. I have to say this change was very hard to get used to. First 3 days I was accidentally pushing `oooooooooooo` key while trying to concentrate on which key I should press while learning new shortcuts. Simply put, I cannot rest my fingers on keyboard like I've used to. But I think that they are not that bad after I got used to them. Sometimes I accidentally press `Backspace` just before hitting `Enter`. Therefore I'd definitely recommend getting heavier switches for function keys. If I was to make another keyboard, which I am definitely going to make, I will choose something in between. When I try to type on my old keyboard I feel like I have to do push ups with my fingers. But I know it is just because I got used to these light ones.

![Workspace setup at work](/images/uploads/2020-03-23_20-24-18-981.jpg 'Workspace setup at work')

And this is what my setup looks like at work.
