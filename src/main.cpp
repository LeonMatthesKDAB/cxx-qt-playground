#include <QtCore/QDebug>
#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <iostream>

#include "myobject.h"

int
main(int argc, char* argv[])
{
  QGuiApplication app(argc, argv);

  QQmlApplicationEngine engine;

  qmlRegisterType<MyObject>(
    "com.kdab.cxx_qt.demo", 1, 0, "MyObject");

  MyObject obj;

  QObject::connect(&obj, &MyObject::propChanged, [](){
      std::cout << "Hello World" << std::endl;
      });

  obj.add();

  qWarning() << obj.getProp();
  obj.setProp(10);
  qWarning() << obj.getProp();

  return app.exec();
}
