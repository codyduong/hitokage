// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;

        #[qobject]
        #[qml_element]
        #[qproperty(QString, string)]
        type KomorebiPipe = super::KomorebiPipeRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn say_hi(self: &MyObject, string: &QString, number: i32);

        // #[qinvokable]
        // fn data(self: &KomorebiPipe);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}

impl qobject::MyObject {
    /// Increment the number Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous: i32 = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
}

#[derive(Default)]
pub struct KomorebiPipeRust {
    string: QString,
}

impl qobject::KomorebiPipe {
    pub fn pipe(self: Pin<&mut Self>, string: &QString) {
        self.set_string(string.clone())
    }
}
