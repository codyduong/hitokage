import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import hitokage 1.0

Window {
    height: 480
    // flags: Qt.FramelessWindowHint // | Qt.WindowStaysOnTopHint
    title: qsTr("hitokage")
    visible: true
    width: 640

    MyObject {
        id: myObject
        number: 1
        string: qsTr("My String with my number: %1").arg(myObject.number)
    }

    KomorebiPipe {
        id: komorebiPipe
        string: qsTr("No data")
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

    Column {
        Label {
            text: qsTr("String: %1").arg(komorebiPipe.string)
        }
    }
}
