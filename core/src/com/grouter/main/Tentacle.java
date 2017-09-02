package com.grouter.main;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.Vector2;

import java.util.ArrayList;

public class Tentacle {

    private final float angle = -90f;

    private ArrayList<Segment> segments;
    private Vector2 base;

    public Tentacle(float x, float y, int segment_count, float tentacle_length){
        // create segments
        segments = new ArrayList<Segment>();
        segments.add(new Segment(x, y, tentacle_length/(float)segment_count, angle));
        for (int i = 1; i < segment_count; i++){
            segments.add(new Segment(segments.get(i - 1), tentacle_length/(float)segment_count, angle));
        }

        // assign children to all segments (except the last one)
        for (int i = 0; i < segment_count-1; i++){
            segments.get(i).setChild(segments.get(i + 1));
        }

        base = new Vector2(x, y); // tentacle's root
    }

    public void update(){
        segments.get(segments.size() - 1).update(Gdx.input.getX(),Gdx.graphics.getHeight() - Gdx.input.getY());
        for(int i = segments.size() - 2; i >= 0; i--){
            segments.get(i).update();
        }

        segments.get(0).setA(base);
        for(int i = 1; i < segments.size(); i++){
            segments.get(i).setA(segments.get(i).getParentB().x, segments.get(i).getParentB().y);
        }
    }

    public void render(ShapeRenderer shapeRenderer){
        for(int i = 0; i < segments.size(); i++){
            segments.get(i).render(shapeRenderer);
        }
    }

}
