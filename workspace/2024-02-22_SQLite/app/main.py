import sqlalchemy
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column, Integer, String
from sqlalchemy.orm import sessionmaker

Base = declarative_base()


class Item(Base):

    id = Column(Integer, primary_key=True)
    name = Column(String(length=255))

    __tablename__ = 'Item'

def select(session):
    query_result = session.query(Item)
    for item in query_result:
        print(item.id, item.name)

def insert(session, item: Item):
    session.add(instance=item)
    session.commit()

def delete(session, item: Item):
    session.delete(item)
    session.commit()


def main():
    engine = sqlalchemy.create_engine('sqlite:///test.db', echo=True)
    session = sessionmaker(bind=engine)()

    pen = Item()
    pen.id = 2
    pen.name = 'pen'

    select(session)
    insert(session, pen)
    select(session)
    delete(session, pen)
    select(session)
    

if __name__ == '__main__':
    main()
