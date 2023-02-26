#!/bin/bash
# Copyright © 2012-2015 Martin Ueding <dev@martin-ueding.de>

# Stop music playback in Clementine.
if type qdbus && qdbus | grep -i clementine
then
    qdbus org.mpris.clementine /Player Stop
fi

# Try to lock the screen with KDE locker.
if type qdbus && qdbus | grep -i org.freedesktop.ScreenSaver
then
    dbus_parameters='org.freedesktop.ScreenSaver /ScreenSaver Lock'

    if qdbus
    then
        qdbus $dbus_parameters
    elif qdbus -qt4
    then
        qdbus -qt4 $dbus_parameters
    fi
elif type slock
then
    slock &
fi

# Power all screens down.
sleep 1
xset dpms force off
