#!/bin/bash

./appify -author "Yun Sheng" -icon ./assets/icon-256.png -id "ysheng26.bingus.bungus" -name "Cliphist" -version $(git tag --points-at HEAD) ./target/release/cliphist_gui
