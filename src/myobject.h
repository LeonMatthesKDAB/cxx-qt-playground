#pragma once

#include "rust/cxx.h"

#include <QtCore/QObject>

#include <memory>

class MyObjectRust;

class MyObject : public QObject
{
    Q_OBJECT

    Q_PROPERTY(int prop READ getProp WRITE setProp NOTIFY propChanged)

public:
    MyObject(QObject* parent = nullptr);
    const MyObjectRust& rust() const;
    MyObjectRust& rust_mut();

    Q_INVOKABLE void add();

    int getProp() const;
    void setProp(int value);

Q_SIGNALS:
    void propChanged();

private:
    rust::Box<MyObjectRust> m_rust;
};
