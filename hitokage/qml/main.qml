import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// ANCHOR: book_qml_import
// This must match the uri and version
// specified in the qml_module in the build.rs script.
import hitokage 1.0
// ANCHOR_END: book_qml_import

Window {
    height: 480
    flags: Qt.WindowStaysOnTopHint | Qt.FramelessWindowHint
    title: qsTr("Hitokage")
    visible: true
    width: 640

    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(myObject.number)
        }

        Label {
            text: qsTr("String: %1").arg(myObject.string)
        }

        Button {
            text: qsTr("Increment Number")

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }

        Button {
            text: qsTr("Quit")

            onClicked: Qt.quit()
        }
    }
}
// ANCHOR_END: book_main_qml